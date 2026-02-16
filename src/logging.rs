use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use colored::Colorize;
use crate::handlers::AppState;

pub async fn print_request_log(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    if state.silent {
        return next.run(req).await;
    }

    let method = req.method().clone();
    let uri = req.uri().clone();
    
    // Extract User-Agent header safely
    let user_agent = req.headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-");

    // Format ISO8601 time with milliseconds (e.g., 2026-02-16T06:08:54.014Z)
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");

    // Standard log line
    // [TIMESTAMP] "METHOD URI" "UA"
    println!("[{}]  \"{} {}\" \"{}\"", 
        timestamp.to_string().dimmed(), 
        method.as_str().cyan(), 
        uri.to_string().cyan(), 
        user_agent.dimmed()
    );

    // Proceed with the request
    let response = next.run(req).await;

    // Check for error responses (4xx or 5xx)
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        let reason = status.canonical_reason().unwrap_or("Unknown Error");
        let error_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");
        
        let status_color = if status.is_server_error() {
            status.as_u16().to_string().red()
        } else {
            status.as_u16().to_string().yellow()
        };

        println!("[{}]  \"{} {}\" Error ({}): \"{}\"", 
            error_timestamp.to_string().dimmed(), 
            method.as_str().cyan(), 
            uri.to_string().cyan(), 
            status_color, 
            reason.red()
        );
    }

    response
}
