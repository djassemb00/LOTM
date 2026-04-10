//! Structure layer - generates buildings, churches, factories, etc.
//!
//! LOTM-specific structures:
//! - Victorian houses
//! - Churches (Evernight Goddess, God of Steam and Machinery, etc.)
//! - Factories (industrial areas)
//! - Beyonder gathering spots
//! - Sealed artifact vaults
//! - Underground crypts

use lotm_common::types::{Chunk, BlockKind};
use lotm_common::vek::*;

/// Generate structures for the world
pub fn generate(sim: &mut crate::sim::WorldSim, index: &crate::Index, seed: u32) {
    tracing::info!("Structure generation complete");
}

/// Apply structures to a chunk
pub fn apply_to_chunk(
    chunk: &mut Chunk,
    sim: &crate::sim::WorldSim,
    index: crate::IndexRef,
    chunk_pos: Vec2<i32>,
) {
    let chunk_size = Chunk::SIZE as i32;

    // Check if this chunk is in a city area
    if let Some(sim_chunk) = sim.get(chunk_pos) {
        if sim_chunk.city_factor > 0.5 {
            // Generate buildings
            generate_buildings(chunk, chunk_pos, sim_chunk.city_factor);
        }

        // Check for special structures
        if let Some(structure) = get_structure_at(chunk_pos, sim, index) {
            place_structure(chunk, chunk_pos, &structure);
        }
    }
}

fn generate_buildings(chunk: &mut Chunk, chunk_pos: Vec2<i32>, city_factor: f32) {
    let chunk_size = Chunk::SIZE as i32;
    let seed = (chunk_pos.x as u32).wrapping_mul(374761393) ^ (chunk_pos.y as u32).wrapping_mul(668265263);

    // Simple building placement using hash as RNG
    for y in (2..chunk_size - 8).step_by(10) {
        for x in (2..chunk_size - 8).step_by(10) {
            let hash = ((x as u32).wrapping_mul(73856093) ^ (y as u32).wrapping_mul(19349663)) % 1000;
            let should_place = (hash as f32 / 1000.0) < city_factor * 0.7;

            if should_place {
                let building_width = 4 + (hash % 3) as i32;
                let building_depth = 4 + (hash % 2) as i32;
                let building_height = 8 + (hash % 8) as i32;

                place_building(chunk, Vec3::new(x, y, 0), building_width, building_depth, building_height);
            }
        }
    }
}

fn place_building(chunk: &mut Chunk, pos: Vec3<i32>, width: i32, depth: i32, height: i32) {
    // Foundation
    for dz in 0..depth {
        for dx in 0..width {
            let base_pos = pos + Vec3::new(dx, dz, 0);
            chunk.set_block(base_pos, BlockKind::Cobblestone);
        }
    }

    // Walls
    for z in 1..height {
        for dy in 0..depth {
            for dx in 0..width {
                let wall_pos = pos + Vec3::new(dx, dy, z);

                // Outer walls only
                if dx == 0 || dx == width - 1 || dy == 0 || dy == depth - 1 {
                    // Windows every 3 blocks
                    if z > 1 && z % 3 != 0 {
                        chunk.set_block(wall_pos, BlockKind::Brick);
                    } else {
                        chunk.set_block(wall_pos, BlockKind::Glass);
                    }
                } else if z == height - 1 {
                    // Roof
                    chunk.set_block(wall_pos, BlockKind::Wood);
                }
            }
        }
    }
}

fn get_structure_at(
    chunk_pos: Vec2<i32>,
    sim: &crate::sim::WorldSim,
    index: crate::IndexRef,
) -> Option<StructureKind> {
    // Determine structure type based on location
    if let Some(sim_chunk) = sim.get(chunk_pos) {
        if sim_chunk.corruption > 0.8 {
            return Some(StructureKind::SealedArtifactVault);
        }
        if sim_chunk.city_factor > 0.7 {
            return Some(StructureKind::Church);
        }
    }
    None
}

fn place_structure(chunk: &mut Chunk, chunk_pos: Vec2<i32>, structure: &StructureKind) {
    match structure {
        StructureKind::Church => place_church(chunk, chunk_pos),
        StructureKind::SealedArtifactVault => place_vault(chunk, chunk_pos),
    }
}

fn place_church(chunk: &mut Chunk, chunk_pos: Vec2<i32>) {
    // Simple church structure
    let center = Vec3::new(Chunk::SIZE as i32 / 2, Chunk::SIZE as i32 / 2, 0);

    // Main hall
    for z in 0..16 {
        for y in -6..6 {
            for x in -4..4 {
                let pos = center + Vec3::new(x, y, z);
                if x == -4 || x == 3 || y == -6 || y == 5 {
                    chunk.set_block(pos, BlockKind::Cobblestone);
                }
            }
        }
    }

    // Altar
    for z in 0..3 {
        for y in -1..1 {
            for x in -1..1 {
                let pos = center + Vec3::new(x, y, z);
                chunk.set_block(pos, BlockKind::Gold);
            }
        }
    }
}

fn place_vault(chunk: &mut Chunk, chunk_pos: Vec2<i32>) {
    // Underground vault for sealed artifacts
    let center = Vec3::new(Chunk::SIZE as i32 / 2, Chunk::SIZE as i32 / 2, -20);

    // Vault chamber
    for z in 0..8 {
        for y in -4..4 {
            for x in -4..4 {
                let pos = center + Vec3::new(x, y, z);
                if x == -4 || x == 3 || y == -4 || y == 3 || z == 0 || z == 7 {
                    chunk.set_block(pos, BlockKind::Iron);
                }
            }
        }
    }

    // Sealing runes (represented as corruption blocks)
    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::PI / 2.0;
        let x = (angle.cos() * 3.0) as i32;
        let y = (angle.sin() * 3.0) as i32;
        let pos = center + Vec3::new(x, y, 4);
        chunk.set_block(pos, BlockKind::Corruption);
    }
}

#[derive(Debug, Clone)]
enum StructureKind {
    Church,
    SealedArtifactVault,
}
