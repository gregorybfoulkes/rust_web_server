use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode};
use serde_json::json;
use tokio::time::Instant;
use tracing::{debug, instrument};
use uuid::Uuid;
use sys_info;
use chrono;

//NOTES
//use::tokio::time:sleep?


/// Handler for root endpoint
#[instrument(skip_all)]
pub async fn handle_root(request_id: Uuid, start_time: Instant) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let response = json!({
        "message": "Welcome to the Rust Web Server",
        "request_id": request_id.to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "processing_time_ms": start_time.elapsed().as_millis(),
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(response.to_string())))
        .unwrap())
}

// Handler for health check endpoint
#[instrument(skip_all)]
pub async fn handle_health(request_id: Uuid, start: Instant) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let mem_info = sys_info::mem_info().unwrap();
    let response = json!({
        "status": "ok",
        "memory_usage": {
            "total": mem_info.total,
            "free": mem_info.free,
            "available": mem_info.avail,
            "buffers": mem_info.buffers,
            "cached": mem_info.cached,
            "swap_total": mem_info.swap_total,
            "swap_free": mem_info.swap_free,
        },
        "request_id": request_id.to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "processing_time_ms": start.elapsed().as_millis(),
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(response.to_string())))
        .unwrap())
}
