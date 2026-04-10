//! Network handling using QUIC protocol.

/// Network manager
pub struct NetworkManager {
    port: u16,
}

impl NetworkManager {
    pub async fn new(port: u16) -> anyhow::Result<Self> {
        tracing::info!("Network manager initialized on port {}", port);

        Ok(Self {
            port,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
