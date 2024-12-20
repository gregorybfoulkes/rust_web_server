use crate::models::{CreateTask, UpdateTask, Task};
use crate::store::Store;
use hyper::{Request, Response, StatusCode, body::Incoming};
use bytes::Bytes;
use http_body_util::{Full, BodyExt};
use serde_json::json;
use std::sync::Arc;
use tokio::time::Instant;
use uuid::Uuid;
use tracing::instrument;

/// Handler for creating a new task.
///
/// This function takes in a hyper request, a Store instance, a request ID, and
/// a start time.  It returns a hyper response with a JSON body containing the
/// newly created task's ID, as well as a message, request ID, timestamp, and
/// processing time.
///
/// If the request body is invalid, the function returns a 400 Bad Request
/// response with an error message.
///
/// The function is instrumented with tracing.
#[instrument(skip_all)]
pub async fn handle_create_task(
    mut req: Request<Incoming>,
    store: Arc<Store>,
    request_id: Uuid,
    start_time: Instant,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let body = req
        .body_mut()
        .collect()
        .await?
        .to_bytes();
    let task_data = match serde_json::from_slice::<CreateTask>(&body) {
        Ok(task) => task,
        Err(_) => {
            return respond_with_error(
                "Invalid request body",
                request_id,
                start_time,
                StatusCode::BAD_REQUEST,
            );
        }
    };

    let new_task_id = store.create_task(task_data).await;
    let response = json!({
        "id": new_task_id,
        "message": "Task created successfully",
        "request_id": request_id.to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "processing_time_ms": start_time.elapsed().as_millis(),
    });
    let bytes = Bytes::from(response.to_string());
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .body(Full::new(bytes))
        .unwrap())
}

// Handler for updating a task
#[instrument(skip(store, req))]
pub async fn handle_update_task(
    mut req: Request<Incoming>,
    store: Arc<Store>,
    task_id_str: &str,
    request_id: Uuid,
    start_time: Instant,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let task_id = match task_id_str.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return respond_with_error("Invalid task ID", request_id, start_time, StatusCode::BAD_REQUEST),
    };

    let body_bytes = req.body_mut().collect().await?.to_bytes();
    let update_data: UpdateTask = match serde_json::from_slice(&body_bytes) {
        Ok(data) => data,
        Err(_) => return respond_with_error("Invalid request body", request_id, start_time, StatusCode::BAD_REQUEST),
    };

    match store.update_task(task_id, update_data).await {
        Some(_) => {
            let response = json!({
                "message": "Task updated successfully",
                "request_id": request_id.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "processing_time_ms": start_time.elapsed().as_millis()
            });
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(response.to_string())))
                .unwrap())
        }
        None => respond_with_error("Task not found", request_id, start_time, StatusCode::NOT_FOUND),
    }
}

/// Return a response with an error message and the given status code.
///
/// The response will also contain the request ID and a timestamp.
fn respond_with_error(
    error: &str,
    request_id: Uuid,
    start_time: Instant,
    status: StatusCode,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let response = json!({
        "error": error,
        "request_id": request_id.to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "processing_time_ms": start_time.elapsed().as_millis(),
    });
    let body = Bytes::from(response.to_string());

    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Full::new(body))
        .unwrap())
}

// Handler for deleting a task
pub async fn handle_delete_task(
    store: Arc<Store>,
    task_id_str: &str,
    request_id: Uuid,
    start_time: Instant,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let task_id = match task_id_str.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let error_response = json!({
                "error": "Invalid task ID",
                "request_id": request_id.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "processing_time_ms": start_time.elapsed().as_millis()
            });
            let error_bytes = Bytes::from(error_response.to_string());
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Full::new(error_bytes))
                .unwrap());
        }
    };

    match store.delete_task(task_id).await {
        Some(_) => {
            let success_response = json!({
                "message": "Task deleted successfully",
                "request_id": request_id.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "processing_time_ms": start_time.elapsed().as_millis()
            });
            let success_bytes = Bytes::from(success_response.to_string());
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Full::new(success_bytes))
                .unwrap())
        }
        None => {
            let error_response = json!({
                "error": "Task not found",
                "request_id": request_id.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "processing_time_ms": start_time.elapsed().as_millis()
            });
            let error_bytes = Bytes::from(error_response.to_string());
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("content-type", "application/json")
                .body(Full::new(error_bytes))
                .unwrap())
        }
    }
}

// Handler for listing all tasks
pub async fn handle_list_tasks(store: Arc<Store>, request_id: Uuid) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let start_time = Instant::now();
    let tasks = store.list_tasks().await;

    let response = json!({
        "tasks": tasks,
        "request_id": request_id.to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "processing_time_ms": start_time.elapsed().as_millis()
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(response.to_string())))
        .unwrap())
}

pub async fn handle_get_task(
    store: Arc<Store>,
    task_id_str: &str,
    request_id: Uuid,
    start_time: Instant,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let task_id = match task_id_str.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
            let error_response = json!({
                "error": "Invalid task ID",
                "request_id": request_id.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "processing_time_ms": start_time.elapsed().as_millis()
            });
            let error_bytes = Bytes::from(error_response.to_string());
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Full::new(error_bytes))
                .unwrap());
        }
    };

    match store.get_task(task_id).await {
        Some(task) => {
            let success_response = json!({
                "task": task,
                "request_id": request_id.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "processing_time_ms": start_time.elapsed().as_millis()
            });
            let success_bytes = Bytes::from(success_response.to_string());
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Full::new(success_bytes))
                .unwrap())
        }
        None => {
            let error_response = json!({
                "error": "Task not found",
                "request_id": request_id.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "processing_time_ms": start_time.elapsed().as_millis()
            });
            let error_bytes = Bytes::from(error_response.to_string());
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("content-type", "application/json")
                .body(Full::new(error_bytes))
                .unwrap())
        }
    }
}