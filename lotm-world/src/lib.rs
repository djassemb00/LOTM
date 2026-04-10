//! # LOTM World Generation
//!
//! Procedural world generation system adapted from Veloren,
//! themed for the Lord of the Mysteries universe.
//!
//! ## Architecture
//!
//! ```text
//! World::generate(seed)
//!     ├── WorldSim::generate()    // Base terrain simulation
//!     ├── CivGen::generate()      // Cities and civilizations
//!     ├── CorruptionGen::generate() // Fog and corruption zones
//!     └── StructureGen::generate() // Buildings and points of interest
//! ```

pub mod sim;
pub mod layer;
pub mod civ;
pub mod util;
pub mod config;
pub mod index;
pub mod column;
pub mod block;

// Re-exports
pub use config::CONFIG;
pub use index::{Index, IndexOwned, IndexRef};
pub use sim::{WorldSim, SimChunk, WorldOpts};
pub use column::ColumnSample;

use lotm_common::vek::*;

/// Main world structure
pub struct World {
    pub sim: WorldSim,
    pub index: IndexOwned,
}

impl World {
    /// Generate a new world with the given seed
    pub fn generate(
        seed: u32,
        opts: WorldOpts,
        threadpool: &rayon::ThreadPool,
        progress_cb: &(dyn Fn(GenerateStage) + Send + Sync),
    ) -> (Self, IndexOwned) {
        threadpool.install(|| {
            let mut index = Index::new(seed);

            progress_cb(GenerateStage::TerrainSimulation(0.0));

            let sim = WorldSim::generate(seed, opts, threadpool, &|stage| {
                progress_cb(GenerateStage::TerrainSimulation(stage));
            });

            progress_cb(GenerateStage::CityGeneration(0.0));

            let cities = civ::CivGen::generate(seed, &mut sim, &mut index, &|stage| {
                progress_cb(GenerateStage::CityGeneration(stage));
            });

            progress_cb(GenerateStage::CorruptionGeneration(0.0));

            // Generate corruption/fog zones
            layer::corruption::generate(&mut sim, &index, seed);

            progress_cb(GenerateStage::StructureGeneration(0.0));

            // Generate structures (churches, factories, etc.)
            layer::structure::generate(&mut sim, &index, seed);

            progress_cb(GenerateStage::Complete(1.0));

            (
                Self {
                    sim,
                    index: index.clone(),
                },
                index,
            )
        })
    }

    /// Get a reference to the world simulation
    pub fn sim(&self) -> &WorldSim {
        &self.sim
    }

    /// Generate a single chunk at the given position
    pub fn generate_chunk(
        &self,
        index: IndexRef,
        chunk_pos: Vec2<i32>,
        should_continue: impl FnMut() -> bool,
    ) -> Result<lotm_common::types::Chunk, String> {
        // Implementation adapted from Veloren's chunk generation
        self.generate_chunk_impl(index, chunk_pos, should_continue)
    }

    fn generate_chunk_impl(
        &self,
        index: IndexRef,
        chunk_pos: Vec2<i32>,
        mut should_continue: impl FnMut() -> bool,
    ) -> Result<lotm_common::types::Chunk, String> {
        use lotm_common::types::{Chunk, BlockKind, CPos};

        let mut chunk = Chunk::new(chunk_pos);

        // Generate blocks using column sampling
        let column_gen = column::ColumnGen::new(&self.sim);

        for y in 0..Chunk::SIZE {
            for x in 0..Chunk::SIZE {
                if should_continue() {
                    return Err("Chunk generation interrupted".to_string());
                }

                let world_pos = chunk_pos * Chunk::SIZE as i32 + Vec2::new(x as i32, y as i32);

                if let Some(col) = column_gen.get((world_pos, index, None)) {
                    // Fill vertical column
                    for z in 0..Chunk::HEIGHT as i32 {
                        let block_kind = block::get_block_at(&col, z, &index);
                        chunk.set_block(Vec3::new(x as i32, y as i32, z), block_kind);
                    }
                }
            }
        }

        // Apply layers
        layer::apply_all(&mut chunk, &self.sim, index, chunk_pos);

        Ok(chunk)
    }
}

/// Generation stage for progress reporting
#[derive(Debug, Clone, Copy)]
pub enum GenerateStage {
    TerrainSimulation(f64),
    CityGeneration(f64),
    CorruptionGeneration(f64),
    StructureGeneration(f64),
    Complete(f64),
}
