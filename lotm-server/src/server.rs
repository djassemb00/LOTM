//! Main server state.

use anyhow::Result;

pub struct Server {
    running: bool,
    port: u16,
}

impl Server {
    pub async fn new() -> Result<Self> {
        let port = std::env::var("LOTM_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(14242);

        tracing::info!("Server configured on port {}", port);

        Ok(Self {
            running: true,
            port,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Server listening on port {}...", self.port);

        while self.running {
            // Handle connections
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }
}
