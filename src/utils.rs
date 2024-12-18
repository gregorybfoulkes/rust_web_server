use hyper::Request;
use tracing::debug;
use tracing::instrument;

// Parse the request body into the specified type
#[instrument(skip(req), err)]
pub async fn parse_body<T: serde::de::DeserializeOwned>(req: Request<hyper::body::Incoming>) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    use http_body_util::BodyExt;
    let body = req.into_body();
    let bytes = body.collect().await.map_err(|e| format!("Failed to collect request body: {}", e))?.to_bytes();
    debug!(body_size = bytes.len(), "Received request body");
    serde_json::from_slice(&bytes).map_err(|e| format!("Failed to parse request body: {}", e).into())
}

// Initialize tracing and error handling
pub fn setup() -> color_eyre::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .json()
        .flatten_event(true)
        .try_init()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to initialize tracing: {}", e))?;

    color_eyre::install().map_err(|e| color_eyre::eyre::eyre!("Failed to install color_eyre: {}", e))?;
    
    debug!("Logging initialized successfully");
    Ok(())
}
