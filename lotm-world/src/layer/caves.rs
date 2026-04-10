//! Caves layer - generates underground passages and chambers.
//!
//! LOTM caves include:
//! - Natural caverns
//! - Underground crypts and tombs
//! - Beyonder secret hideouts
//! - Ancient ruins

use lotm_common::types::{Chunk, BlockKind};
use lotm_common::vek::*;
use noise::NoiseFn;

/// Apply caves to a chunk
pub fn apply_to_chunk(
    chunk: &mut Chunk,
    sim: &crate::sim::WorldSim,
    index: crate::IndexRef,
    chunk_pos: Vec2<i32>,
) {
    let chunk_size = Chunk::SIZE as i32;
    let seed = index.seed as f64;

    for z in 0..Chunk::HEIGHT as i32 {
        // Only generate caves underground
        if z > 100 {
            continue;
        }

        for y in 0..chunk_size {
            for x in 0..chunk_size {
                let world_pos = chunk_pos * chunk_size + Vec2::new(x, y);
                let pos_3d = Vec3::new(x, y, z);

                // Cave noise
                let cave_noise = noise::SuperSimplex::new(42).get([
                    world_pos.x as f64 / 32.0,
                    world_pos.y as f64 / 32.0,
                    z as f64 / 16.0,
                ]);

                // Create caves where noise is high
                if cave_noise > 0.3 {
                    chunk.set_block(pos_3d, BlockKind::Air);
                }

                // Special underground structures at certain depths
                if z < 50 && cave_noise > 0.5 {
                    // Ancient ruins or crypts
                    if let Some(sim_chunk) = sim.get(world_pos.map(|e| e.div_euclid(chunk_size))) {
                        if sim_chunk.corruption > 0.6 {
                            // Corrupted underground chamber
                            place_corrupted_chamber(chunk, pos_3d, index.seed);
                        }
                    }
                }
            }
        }
    }
}

fn place_corrupted_chamber(chunk: &mut Chunk, pos: Vec3<i32>, seed: u32) {
    // Small corrupted chamber underground
    let hash = (pos.x as u32).wrapping_mul(73856093) ^ (pos.y as u32).wrapping_mul(19349663);

    if hash % 100 < 5 {
        // Place corruption altar
        for dz in 0..3 {
            for dy in -1..2 {
                for dx in -1..2 {
                    let altar_pos = pos + Vec3::new(dx, dy, dz);
                    chunk.set_block(altar_pos, BlockKind::Corruption);
                }
            }
        }
    }
}
