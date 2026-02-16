use clap::Parser;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "zserv: A simple, robust HTTP server in Rust", long_about = None)]
pub struct Config {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Address to bind to
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: IpAddr,

    /// Directory to serve
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Enable CORS headers
    #[arg(long, default_value_t = false)]
    pub cors: bool,

    /// Suppress log output
    #[arg(short, long, default_value_t = false)]
    pub silent: bool,
}
