use crate::handlers::*;
use crate::store::Store;
use crate::models::{CreateTask, UpdateTask};
use hyper::{Request, Response, StatusCode, body::Incoming};
use serde_json::Value;
use tokio::time::Instant;
use uuid::Uuid;
use std::sync::Arc;
use bytes::Bytes;
use http_body_util::{Full, BodyExt};
use hyper::body::Body;

fn create_json_request<T: serde::Serialize>(method: hyper::Method, uri: &str, body: &T) -> Request<Incoming> {
    let body_str = serde_json::to_string(body).unwrap();
    let bytes = Bytes::from(body_str);
    let full_body = Full::new(bytes);
    let boxed_body = full_body.map_err(|_| std::io::Error::new(
        std::io::ErrorKind::Other,
        "body conversion failed"
    )).boxed();
    
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(boxed_body)
        .unwrap()
}

async fn get_body_json(response: Response<Full<Bytes>>) -> Value {
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body_bytes).unwrap()
}

#[tokio::test]
async fn test_handle_create_task() {
    let store = Arc::new(Store::new());
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    let create_task = CreateTask {
        title: "Test Task".to_string(),
        description: "Test Description".to_string(),
    };

    let request = create_json_request(hyper::Method::POST, "/tasks", &create_task);
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
    let response = handle_list_tasks(store.clone(), request_id, start_time).await.unwrap();

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
