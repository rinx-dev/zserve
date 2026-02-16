use axum::{
    Router,
    middleware,
    routing::get,
    http::Method,
};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};
use std::time::Duration;
use crate::cli::Config;
use crate::handlers::{handle_request, AppState};
use crate::logging::print_request_log;

#[allow(deprecated)]
pub async fn build_app(config: Config) -> Router {
    // Resolve absolute path for serving
    let root_path = std::fs::canonicalize(&config.path).unwrap_or_else(|_| {
        eprintln!("Warning: Could not canonicalize path {:?}, using as-is", config.path);
        config.path.clone()
    });

    let state = AppState {
        root_path,
        silent: config.silent,
    };

    let mut router = Router::new()
        // Fallback for directory listing and file serving
        .fallback(get(handle_request).post(handle_request)) // Handle GET and POST mainly, HEAD is handled by GET usually
        .with_state(state.clone());

    // Add middleware
    router = router.layer(middleware::from_fn_with_state(state, print_request_log));
    router = router.layer(CompressionLayer::new());
    
    // Set 120s timeout to match the banner claim
    router = router.layer(TimeoutLayer::new(Duration::from_secs(120)));

    if config.cors {
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::HEAD, Method::OPTIONS])
            .allow_origin(Any);
        router = router.layer(cors);
    }

    router
}
