//! Main game state and loop.

use anyhow::Result;
use crate::camera::Camera;

pub struct Game {
    running: bool,
    camera: Camera,
}

impl Game {
    pub async fn new() -> Result<Self> {
        tracing::info!("Initializing game...");

        Ok(Self {
            running: true,
            camera: Camera::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Game loop starting...");

        while self.running {
            // Update
            self.update().await?;

            // Render
            self.render().await?;

            // Small delay to prevent busy loop
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
        }

        Ok(())
    }

    async fn update(&mut self) -> Result<()> {
        // Update camera
        self.camera.update(glam::Vec3::new(0.0, 0.0, 0.0));
        Ok(())
    }

    async fn render(&mut self) -> Result<()> {
        // Render frame (placeholder)
        Ok(())
    }
}
