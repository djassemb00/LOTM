//! # LOTM Server
//!
//! Multiplayer server for Lord of the Mysteries game.

use tracing_subscriber::EnvFilter;

mod server;
mod network;
mod world_manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("LOTM Server starting...");

    // Initialize server
    let mut server = server::Server::new().await?;

    // Run server
    server.run().await
}
