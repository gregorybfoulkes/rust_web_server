use crate::handlers::*;
use hyper::{Request, Response, StatusCode, body::Incoming};
use serde_json::Value;
use tokio::time::Instant;
use uuid::Uuid;
use bytes::Bytes;
use http_body_util::{Full, BodyExt};

async fn get_body_json(response: Response<Full<Bytes>>) -> Value {
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body_bytes).unwrap()
}

#[tokio::test]
async fn test_handle_root() {
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
