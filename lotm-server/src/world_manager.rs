//! World management on server side.

use lotm_world::World;

/// Manages the game world on server
pub struct WorldManager {
    world: Option<World>,
    seed: u32,
}

impl WorldManager {
    pub fn new(seed: u32) -> Self {
        Self {
            world: None,
            seed,
        }
    }

    pub async fn generate(&mut self) -> anyhow::Result<()> {
        tracing::info!("Generating world with seed {}...", self.seed);

        let threadpool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .build()?;

        let opts = lotm_world::sim::WorldOpts::default();

        let (world, index) = World::generate(
            self.seed,
            opts,
            &threadpool,
            &|stage| {
                tracing::info!("Generation stage: {:?}", stage);
            },
        );

        self.world = Some(world);

        tracing::info!("World generation complete!");

        Ok(())
    }
}
