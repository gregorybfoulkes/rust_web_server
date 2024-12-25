use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use tokio::net::TcpListener;
use tokio::time::Instant;
use uuid::Uuid;
use hyper_util::rt::TokioIo;

use crate::store::Store;
use http_body_util::Full;
use bytes::Bytes;
use hyper::body::Incoming;

mod handlers;
mod models;
mod store;
mod utils;

use handlers::*;

/// Router for the web server.
///
/// This function takes in a hyper request and matches on the method and path to
/// call the corresponding handler.  If the request is invalid, it returns a 404
/// Not Found response.  If the handler returns an error, it returns a 500
/// Internal Server Error response.
/// 
/// This is the main entrypoint for the web server.  It sets up a TCP listener on
/// localhost port 3000 and spawns a new task for each incoming connection.  Each
/// task serves the connection using the `router` function, which calls the
/// corresponding handler for the request.
///
/// The handler functions are in the `handlers` module.  The router is used in
/// the `main` function to create a hyper service.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = TcpListener::bind(addr).await?;
    let store = Arc::new(Store::new());
    println!("Server running on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let store = Arc::clone(&store);
        
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| router(req, store.clone())))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }
}

/// The router function takes in a hyper request and matches on the method and
/// path to call the corresponding handler.  If the request is invalid, it
/// returns a 404 Not Found response.  If the handler returns an error, it
/// returns a 500 Internal Server Error response.


async fn router(
    req: Request<Incoming>,
    store: Arc<Store>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let request_id = Uuid::new_v4();
    let start = Instant::now();
    
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    
    // Extract task_id from path if present
    let task_id = if path.starts_with("/tasks/") {
        Some(path.trim_start_matches("/tasks/").to_string())
    } else {
        None
    };

    let result = match (method, path.as_str()) {
        (Method::GET, "/") => handle_root(request_id, start).await,
        (Method::GET, "/health") => handle_health(request_id, start).await,
        (Method::POST, "/tasks") => handle_create_task(req, store, request_id, start).await,
        (Method::PUT, _) if task_id.is_some() => {
            handle_update_task(req, store, &task_id.unwrap(), request_id, start).await
        }
        (Method::DELETE, _) if task_id.is_some() => {
            handle_delete_task(store, &task_id.unwrap(), request_id, start).await
        }
        (Method::GET, "/tasks") => handle_list_tasks(store, request_id, start).await,
        _ => Ok(Response::builder()
            .status(404)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap()),
    };

    Ok(result.unwrap_or_else(|err| {
        Response::builder()
            .status(500)
            .body(Full::new(Bytes::from(format!("Internal Server Error: {}", err))))
            .unwrap()
    }))
} 
