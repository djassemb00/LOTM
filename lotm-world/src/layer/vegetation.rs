//! Vegetation layer - generates trees, plants, etc.
//!
//! In LOTM, vegetation is sparse in corrupted areas but lush in normal zones.

use lotm_common::types::{Chunk, BlockKind};
use lotm_common::vek::*;

/// Apply vegetation to a chunk
pub fn apply_to_chunk(
    chunk: &mut Chunk,
    sim: &crate::sim::WorldSim,
    index: crate::IndexRef,
    chunk_pos: Vec2<i32>,
) {
    let chunk_size = Chunk::SIZE as i32;

    if let Some(sim_chunk) = sim.get(chunk_pos) {
        let tree_density = sim_chunk.tree_density;

        // Skip if no trees
        if tree_density < 0.1 {
            return;
        }

        // Place trees
        for y in 0..chunk_size {
            for x in 0..chunk_size {
                let hash = ((x as u32).wrapping_mul(73856093) ^ (y as u32).wrapping_mul(19349663)) % 1000;
                let should_place = (hash as f32 / 1000.0) < tree_density;

                if should_place {
                    let local_pos = Vec3::new(x, y, 0);
                    // Find surface
                    if let Some(surface_z) = find_surface(chunk, x, y) {
                        let tree_pos = Vec3::new(x, y, surface_z);
                        place_tree(chunk, tree_pos, hash % 100);
                    }
                }
            }
        }
    }
}

fn find_surface(chunk: &Chunk, x: i32, y: i32) -> Option<i32> {
    for z in (0..Chunk::HEIGHT as i32).rev() {
        if chunk.get_block(Vec3::new(x, y, z)).map_or(false, |b| b.is_solid()) {
            return Some(z + 1);
        }
    }
    None
}

fn place_tree(chunk: &mut Chunk, pos: Vec3<i32>, variant: u32) {
    let height = 5 + (variant % 4) as i32;

    // Trunk
    for z in 0..height {
        chunk.set_block(pos + Vec3::new(0, 0, z), BlockKind::Wood);
    }

    // Leaves
    for dz in -2..3 {
        for dy in -2..3 {
            for dx in -2..3 {
                let leaf_pos = pos + Vec3::new(dx, dy, height + dz);
                let dist = (dx * dx + dy * dy + dz * dz) as f32;
                if dist < 6.0 {
                    chunk.set_block(leaf_pos, BlockKind::Grass);
                }
            }
        }
    }
}
