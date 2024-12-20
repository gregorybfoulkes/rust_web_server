use crate::handlers::*;
use hyper::{body::Incoming, Request, Response, StatusCode};
use serde_json::Value;
use tokio::time::Instant;
use uuid::Uuid;
use bytes::Bytes;
use http_body_util::{Full, BodyExt};

/// Deserialize the body of a hyper::Response into a serde_json::Value.
///
/// The response body is collected and then deserialized into a serde_json::Value.
///
/// # Panics
/// If the body cannot be deserialized into a serde_json::Value, the function will panic.
async fn get_body_json(response: Response<Full<Bytes>>) -> Value {
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body_bytes).unwrap()
}

#[tokio::test]
/*************  ✨ Codeium Command ⭐  *************/
/// Test that the root endpoint returns a 200 OK response with a JSON body
/// containing a message, request ID, timestamp, and processing time.
///
/// Verifies that:
///
/// * The response status is 200 OK.
/// * The response body is JSON with the expected structure.
/// * The response body contains the expected request ID, timestamp, and processing time.
/******  57f94aa2-039d-47f7-97fc-9ba4f1dc3e9c  *******/async fn test_handle_root() {
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    let response = handle_root(request_id, start_time).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert_eq!(body["message"], "Welcome to the Rust Web Server");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
}

/// Test that the health endpoint returns a 200 OK response with a JSON body
/// containing a message, request ID, timestamp, processing time, and memory usage.
///
/// Verifies that:
///
/// * The response status is 200 OK.
/// * The response body is JSON with the expected structure.
/// * The response body contains the expected request ID, timestamp, and processing time.
/// * The response body contains the expected memory usage information.
#[tokio::test]
async fn test_handle_health() {
    let request_id = Uuid::new_v4();
    let start_time = Instant::now();

    let response = handle_health(request_id, start_time).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body = get_body_json(response).await;

    assert_eq!(body["status"], "ok");
    assert!(body["request_id"].is_string());
    assert!(body["timestamp"].is_string());
    assert!(body["processing_time_ms"].is_number());
    assert!(body["memory_usage"].is_object());
}
