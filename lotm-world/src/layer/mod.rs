//! World layers - applied during chunk generation.

pub mod corruption;
pub mod structure;
pub mod vegetation;
pub mod caves;

use lotm_common::types::Chunk;
use lotm_common::vek::*;

/// Apply all layers to a chunk
pub fn apply_all(
    chunk: &mut Chunk,
    sim: &crate::sim::WorldSim,
    index: crate::IndexRef,
    chunk_pos: Vec2<i32>,
) {
    // Apply corruption fog
    corruption::apply_to_chunk(chunk, sim, index, chunk_pos);

    // Apply structures (buildings, churches, etc.)
    structure::apply_to_chunk(chunk, sim, index, chunk_pos);

    // Apply vegetation
    vegetation::apply_to_chunk(chunk, sim, index, chunk_pos);

    // Apply caves (underground)
    caves::apply_to_chunk(chunk, sim, index, chunk_pos);
}
