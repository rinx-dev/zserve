use axum::{
    extract::{State, Request},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use std::path::{Path, PathBuf};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use crate::html::{generate_directory_listing, DirEntry};
use std::fs;
use percent_encoding::percent_decode_str;

#[derive(Clone)]
pub struct AppState {
    pub root_path: PathBuf,
    pub silent: bool,
}

pub async fn handle_request(
    State(state): State<AppState>,
    req: Request,
) -> Result<Response, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let uri = &parts.uri;
    let path = uri.path();
    
    // Decode path
    let decoded_path = percent_decode_str(path)
        .decode_utf8()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid UTF-8 in path: {}", e)))?;

    // Resolve path securely
    // Use the resolve_path helper from below
    // We assume resolve_path is defined in the same file or imported
    // Since we are replacing the function, we keep the helper call
    let full_path = resolve_path(&state.root_path, &decoded_path);

    // If path is invalid (traversal attempt), return 403 or 404
    // Wait, resolve_path returns Option<PathBuf>
    let full_path = full_path.ok_or((StatusCode::FORBIDDEN, "Access denied".to_string()))?;

    // Check if it's a directory
    if full_path.is_dir() {
        // Check for index.html
        let index_path = full_path.join("index.html");
        if !index_path.exists() {
            // Generate directory listing
            return serve_directory_listing(&state.root_path, &full_path, path);
        }
    }

    // Reconstruct request for ServeDir
    let req = Request::from_parts(parts, body);

    // Fallback to ServeDir for files and index.html
    // Note: We create a new ServeDir for each request to ensure we respect the root
    let service = ServeDir::new(&state.root_path);
    
    match service.oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error serving file: {}", err))),
    }
}

fn resolve_path(base: &Path, uri_path: &str) -> Option<PathBuf> {
    // Determine if we need to trim the leading slash.
    // URI paths always start with /.
    let path = uri_path.trim_start_matches('/');
    
    let mut full_path = base.to_path_buf();
    
    // We iterate over the path segments.
    // We must manually split by '/' because Path::new() behavior with '/' on Windows 
    // can be inconsistent if the path is not verbatim.
    // However, simplest cross-platform way for URL paths is split('/').
    for part in path.split('/') {
        if part.is_empty() || part == "." {
            continue;
        } else if part == ".." {
            full_path.pop();
        } else {
            full_path.push(part);
        }
    }

    // Check if it's still under base (or equal to base)
    if full_path.starts_with(base) {
        Some(full_path)
    } else {
        None
    }
}

fn serve_directory_listing(base_path: &Path, full_path: &Path, request_path: &str) -> Result<Response, (StatusCode, String)> {
    let mut entries = Vec::new();

    match fs::read_dir(full_path) {
        Ok(read_dir) => {
            for entry_result in read_dir {
                if let Ok(entry) = entry_result {
                    let metadata = entry.metadata().ok();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
                    let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                    let modified = metadata.as_ref().and_then(|m| m.modified().ok()).map(chrono::DateTime::from);

                    entries.push(DirEntry {
                        name,
                        is_dir,
                        size,
                        modified,
                    });
                }
            }
        },
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read directory: {}", e))),
    }

    // Sort: Directories first, then alphabetical
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    let html = generate_directory_listing(base_path, request_path, &entries);
    Ok(Html(html).into_response())
}
