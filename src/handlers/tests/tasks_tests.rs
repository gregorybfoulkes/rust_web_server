use crate::handlers::{handle_create_task, handle_update_task, handle_delete_task, handle_list_tasks};
use crate::handlers::handle_get_task;
use crate::store::Store;
use crate::models::{CreateTask, UpdateTask};
use hyper::{Request, Response, StatusCode};

use serde_json::Value;
use tokio::time::Instant;
use uuid::Uuid;
use std::sync::Arc;
use bytes::Bytes;
use http_body_util::{Full, BodyExt};
/**
Create a hyper::Request from the given method, uri, and body. The body is serialized to json
before being added to the request. The request is given a content-type header of application/json.

# Panics
If the body cannot be serialized to json, the function will panic.

# Returns
The created request.
**/
fn create_json_request<T: serde::Serialize>(method: hyper::Method, uri: &str, body: &T) -> Request<hyper::body::Body> {
    let to_string = serde_json::to_string(body)?;
    let body_str = to_string;
    let bytes = Bytes::from(body_str);
    let full_body = hyper::body::Body::from(bytes);

    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(full_body)?;

    request
}

#[tokio::test]
async fn test_handle_get_nonexistent_task() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();


    // First create a task
    let create_task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };
    let id = store.create_task(create_task).await;

    // Then get it
    let request = create_json_request(hyper::Method::GET, &format!("/tasks/{}", id), &());
    let response = handle_list_tasks(store.clone(), request_id).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert_eq!(body["id"], id.to_string());
    assert_eq!(body["title"], "Test Task");
    assert_eq!(body["description"], "Test Description");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
}

#[tokio::test]
async fn test_handle_get_task() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    // Try to get a nonexistent task
    let request = create_json_request(hyper::Method::GET, "/tasks/nonexistent", &());
    let response = handle_get_task(store.clone(), request, "nonexistent", request_id).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert_eq!(body["message"], "Task not found");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
}

#[tokio::test]
async fn test_handle_create_task_invalid_body() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    // Create a task with an invalid body
    let invalid_body = "invalid body";
    let request = create_json_request(hyper::Method::POST, "/tasks", &invalid_body).unwrap();
    let response = handle_create_task(request, store.clone(), request_id, start_time).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert_eq!(body["message"], "Invalid request body");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
}

/// Deserialize the body of a hyper::Response into a serde_json::Value.
///
/// The response body is collected and then deserialized into a serde_json::Value.
///
/// # Panics
/// If the body cannot be deserialized into a serde_json::Value, the function will panic.
async fn get_body_json(response: Response<Full<Bytes>>) -> Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

/// Test that a task can be created successfully.
///
/// Verifies that:
///
/// * The response status is 201 Created.
/// * The response body is JSON with the expected structure.
/// * The response body contains the expected request ID, timestamp, and processing time.
///
/// # Panics
/// If the response body cannot be deserialized into a serde_json::Value, the function will panic.
#[tokio::test]
async fn test_handle_create_task() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    let create_task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };

    let request = create_json_request(hyper::Method::POST, "/tasks", &create_task).unwrap();
    let response = handle_create_task(request, store.clone(), request_id, start_time).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert!(body["id"].is_number());
    assert_eq!(body["message"], "Task created successfully");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
}

    /// Test that a task can be updated successfully.
    ///
    /// Verifies that:
    ///
    /// * The response status is 200 OK.
    /// * The response body is JSON with the expected structure.
    /// * The response body contains the expected request ID, timestamp, and processing time.
    ///
#[tokio::test]
async fn test_handle_update_task() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    // First create a task
    let create_task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };
    let id = store.create_task(create_task).await;

    // Then update it
    let update_task = UpdateTask {
        title: Some("Updated Task".to_string()),
        description: Some("Updated Description".to_string()),
        completed: Some(true),
    };

    let request = create_json_request(hyper::Method::PUT, &format!("/tasks/{}", id), &update_task);
    let response = handle_update_task(request, store.clone(), &id.to_string(), request_id, start_time).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert_eq!(body["message"], "Task updated successfully");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
}

    /// Test that a task can be deleted successfully.
    ///
    /// Verifies that:
    ///
    /// * The response status is 200 OK.
    /// * The response body is JSON with the expected structure.
    /// * The response body contains the expected request ID, timestamp, and processing time.
    /// * The task is deleted and subsequent deletion attempts result in a 404 Not Found response.
#[tokio::test]
async fn test_handle_delete_task() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    // First create a task
    let create_task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };
    let id = store.create_task(create_task).await;

    // Then delete it
    let response = handle_delete_task(store.clone(), &id.to_string(), request_id, start_time).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert_eq!(body["message"], "Task deleted successfully");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());

    // Verify task is deleted
    let response = handle_delete_task(store.clone(), &id.to_string(), request_id, start_time).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_handle_list_tasks() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    // First create some tasks
    let create_task1 = CreateTask {
        title: "Test Task 1".to_string(),
        description: "Test Description 1".to_string(),
    };
    let create_task2 = CreateTask {
        title: "Test Task 2".to_string(),
        description: "Test Description 2".to_string(),
    };
    store.create_task(create_task1).await;
    store.create_task(create_task2).await;

    // Then list them
    let response = handle_list_tasks(store.clone(), request_id).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert!(body["tasks"].is_array());
    let tasks = body["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 2);
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
}

#[tokio::test]
async fn test_invalid_task_id() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    let update_task = UpdateTask {
        title: Some("Updated Task".to_string()),
        description: Some("Updated Description".to_string()),
        completed: Some(true),
    };

    // Test invalid task ID for update
    let request = create_json_request(hyper::Method::PUT, "/tasks/invalid", &update_task);
    let response = handle_update_task(request, store.clone(), "invalid", request_id, start_time).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test invalid task ID for delete
    let response = handle_delete_task(store.clone(), "invalid", request_id, start_time).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
