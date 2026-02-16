use clap::Parser;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::cli::Config;
use crate::server::build_app;

mod cli;
mod server;
mod handlers;
mod html;
mod logging;
mod banner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse arguments
    let config = Config::parse();

    // Initialize logging (for libraries mainly, as we have custom request logging)
    if !config.silent {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Suppress our own crate's info logs since we use banner/custom log
                // Keep errors/warns
                "zserv=warn,info".into()
            }))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    let addr = SocketAddr::from((config.address, config.port));
    
    // Build application
    let app = build_app(config.clone()).await;

    // Start server
    if !config.silent {
        crate::banner::print_banner(&config, addr);
    }

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
