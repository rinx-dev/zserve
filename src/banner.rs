use crate::cli::Config;
use colored::Colorize;
use local_ip_address::list_afinet_netifas;
use std::net::SocketAddr;

pub fn print_banner(config: &Config, addr: SocketAddr) {
    let app_name = env!("CARGO_PKG_NAME").yellow();
    let version = env!("CARGO_PKG_VERSION");

    // Resolve display path
    let canonical_path = std::fs::canonicalize(&config.path).unwrap_or(config.path.clone());
    let path_str = canonical_path.to_string_lossy();
    let display_path = if path_str.starts_with(r"\\?\") {
        &path_str[4..]
    } else {
        &path_str
    };

    println!("Starting up {}, serving {}", app_name, display_path.cyan());
    println!();
    println!("{} version: {}", app_name, version.bright_white());
    println!();
    println!("{} settings:", app_name);
    println!(
        "CORS: {}",
        if config.cors {
            "enabled".green()
        } else {
            "disabled".red()
        }
    );
    println!("Cache: {}", "disabled".red());
    println!("Connection Timeout: {}", "120 seconds".bright_white());
    println!("Directory Listings: {}", "visible".green());
    println!("AutoIndex: {}", "visible".green());
    println!("Serve GZIP Files: {}", "true".green());
    println!("Serve Brotli Files: {}", "true".green());
    println!("Default File Extension: {}", "none".bright_white());
    println!();
    println!("Available on:");

    // If bound to 0.0.0.0 or ::, list all available interfaces
    if addr.ip().is_unspecified() {
        if let Ok(interfaces) = list_afinet_netifas() {
            for (_, ip) in interfaces {
                // Filter by family (IPv4 vs IPv6) based on binding
                if addr.is_ipv4() && ip.is_ipv4() {
                    let url = format!("http://{}:{}", ip, addr.port());
                    println!("  {}", url.underline().blue());
                } else if addr.is_ipv6() && ip.is_ipv6() {
                    let url = format!("http://[{}]:{}", ip, addr.port());
                    println!("  {}", url.underline().blue());
                }
            }
        } else {
            // Fallback
            if addr.is_ipv4() {
                let url = format!("http://127.0.0.1:{}", addr.port());
                println!("  {}", url.underline().blue());
            } else {
                let url = format!("http://[::1]:{}", addr.port());
                println!("  {}", url.underline().blue());
            }
        }
    } else {
        // Just print the bound address
        let url = format!("http://{}", addr);
        println!("  {}", url.underline().blue());
    }

    println!("Hit CTRL-C to stop the server");
    println!();
}
