//! # LOTM Client
//!
//! Client application for Lord of the Mysteries game.

use tracing_subscriber::EnvFilter;

mod game;
mod camera;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("LOTM Client starting...");

    // Initialize game
    let mut game = game::Game::new().await?;

    // Run game loop
    game.run().await
}
