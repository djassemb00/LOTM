//! Main game state and loop.

use anyhow::Result;

pub struct Game {
    // Game state
    running: bool,
}

impl Game {
    pub async fn new() -> Result<Self> {
        tracing::info!("Initializing game...");

        Ok(Self {
            running: true,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Game loop starting...");

        while self.running {
            // Update
            self.update().await?;

            // Render
            self.render().await?;
        }

        Ok(())
    }

    async fn update(&mut self) -> Result<()> {
        // Update game state
        Ok(())
    }

    async fn render(&mut self) -> Result<()> {
        // Render frame
        Ok(())
    }
}
