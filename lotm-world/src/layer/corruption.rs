//! Corruption/Fog layer - generates corrupted zones and fog barriers.
//!
//! In the LOTM universe, corruption manifests as:
//! - Dense fog that limits visibility
//! - Corrupted terrain blocks
//! - Dangerous zones where Beyonder powers are unstable
//! - Sealed artifact locations

use lotm_common::types::{Chunk, BlockKind};
use lotm_common::vek::*;
use noise::NoiseFn;

/// Generate corruption zones for the entire world
pub fn generate(sim: &mut crate::sim::WorldSim, index: &crate::Index, seed: u32) {
    // Corruption is already calculated in sim chunk generation
    // This function can be used to add additional corruption features
    tracing::info!("Corruption generation complete");
}

/// Apply corruption to a chunk
pub fn apply_to_chunk(
    chunk: &mut Chunk,
    sim: &crate::sim::WorldSim,
    index: crate::IndexRef,
    chunk_pos: Vec2<i32>,
) {
    let chunk_size = Chunk::SIZE as i32;

    for z in 0..Chunk::HEIGHT as i32 {
        for y in 0..chunk_size {
            for x in 0..chunk_size {
                let world_pos = chunk_pos * chunk_size + Vec2::new(x, y);

                if let Some(sim_chunk) = sim.get(world_pos.map(|e| e.div_euclid(chunk_size))) {
                    let corruption = sim_chunk.corruption;

                    // Apply corruption blocks based on level
                    if corruption > 0.5 {
                        let local_pos = Vec3::new(x, y, z);
                        if let Some(block) = chunk.get_block(local_pos) {
                            if block.is_solid() && rand::random::<f32>() < corruption * 0.3 {
                                chunk.set_block(local_pos, BlockKind::Corruption);
                            }
                        }
                    }

                    // Add fog barriers at corruption boundaries
                    if corruption > 0.3 && corruption < 0.7 {
                        let local_pos = Vec3::new(x, y, z);
                        if chunk.get_block(local_pos).map_or(false, |b| b.is_transparent()) {
                            if rand::random::<f32>() < 0.1 {
                                chunk.set_block(local_pos, BlockKind::FogBarrier);
                            }
                        }
                    }
                }
            }
        }
    }
}
