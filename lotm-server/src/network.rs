//! Network handling using QUIC protocol.

use quinn::*;

/// Network manager
pub struct NetworkManager {
    endpoint: Option<Endpoint>,
}

impl NetworkManager {
    pub async fn new(port: u16) -> anyhow::Result<Self> {
        // Create QUIC endpoint
        let endpoint = Endpoint::server(
            ServerConfig::with_crypto(Arc::new(
                rustls::ServerConfig::builder()
                    .with_no_client_auth()
                    .with_certifier(Arc::new(CertifiedKey::default())),
            )),
            SocketAddr::new("0.0.0.0".parse()?, port),
        )?;

        tracing::info!("Network manager initialized");

        Ok(Self {
            endpoint: Some(endpoint),
        })
    }
}
