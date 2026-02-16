use chrono::{DateTime, Local};
use humansize::{format_size, DECIMAL};
use std::path::Path;

pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<DateTime<Local>>,
}

pub fn generate_directory_listing(
    _base_path: &Path,
    request_path: &str,
    entries: &[DirEntry],
) -> String {
    let mut html = String::new();

    // Add header
    html.push_str("<!DOCTYPE html><html><head>");
    html.push_str("<meta charset=\"utf-8\">");
    html.push_str(&format!("<title>Index of {}</title>", request_path));
    html.push_str("<style>");
    html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif; padding: 20px; line-height: 1.5; }");
    html.push_str("table { width: 100%; border-collapse: collapse; }");
    html.push_str("th, td { text-align: left; padding: 8px; border-bottom: 1px solid #ddd; }");
    html.push_str("th { background-color: #f2f2f2; }");
    html.push_str("a { text-decoration: none; color: #0366d6; }");
    html.push_str("a:hover { text-decoration: underline; }");
    html.push_str(".icon { margin-right: 5px; }");
    html.push_str("</style>");
    html.push_str("</head><body>");

    // Add navigation
    html.push_str(&format!("<h1>Index of {}</h1>", request_path));

    // Parent link
    if request_path != "/" {
        let parent = Path::new(request_path).parent().unwrap_or(Path::new("/"));
        let parent_str = parent.to_string_lossy();
        // Ensure parent path starts with /
        let link = if parent_str == "/" || parent_str.is_empty() {
            "/".to_string()
        } else {
            // Fix Windows path separators if they leak, though request_path should be URL path
            parent_str.replace('\\', "/")
        };
        html.push_str(&format!(
            "<p><a href=\"{}\">⬅ Parent Directory</a></p>",
            link
        ));
    }

    // Table
    html.push_str("<table>");
    html.push_str("<thead><tr><th>Name</th><th>Size</th><th>Last Modified</th></tr></thead>");
    html.push_str("<tbody>");

    for entry in entries {
        let name_display = if entry.is_dir {
            format!("{}/", entry.name)
        } else {
            entry.name.clone()
        };

        // Construct link. If request_path is just "/", handled simply.
        // Otherwise, join carefully.
        let link = if request_path == "/" {
            name_display.clone()
        } else {
            // Trim trailing slash from request path for clean join
            let clean_req = request_path.trim_end_matches('/');
            format!("{}/{}", clean_req, name_display)
        };

        let size_display = if entry.is_dir {
            "-".to_string()
        } else {
            format_size(entry.size, DECIMAL)
        };

        let date_display = match entry.modified {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => "-".to_string(),
        };

        let icon = if entry.is_dir { "📁" } else { "📄" };

        html.push_str("<tr>");
        html.push_str(&format!(
            "<td><span class=\"icon\">{}</span><a href=\"{}\">{}</a></td>",
            icon, link, name_display
        ));
        html.push_str(&format!("<td>{}</td>", size_display));
        html.push_str(&format!("<td>{}</td>", date_display));
        html.push_str("</tr>");
    }

    html.push_str("</tbody></table>");
    html.push_str(&format!(
        "<hr><footer><em>zserv - {}</em></footer>",
        Local::now().format("%Y")
    ));
    html.push_str("</body></html>");

    html
}
