//! # LOTM Common
//!
//! Common types and utilities used across all LOTM crates.

use serde::{Deserialize, Serialize};
use vek::*;

/// Block kind identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockKind {
    Air,
    Stone,
    Earth,
    Grass,
    Water,
    Wood,
    Brick,
    Corruption,
}

impl BlockKind {
    pub fn is_solid(&self) -> bool {
        !matches!(self, Self::Air | Self::Water)
    }
}

/// Chunk data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub position: Vec2<i32>,
    pub blocks: Vec<BlockKind>,
}

impl Chunk {
    pub const SIZE: u32 = 32;
    pub const HEIGHT: u32 = 256;

    pub fn new(position: Vec2<i32>) -> Self {
        Self {
            position,
            blocks: vec![BlockKind::Air; Self::SIZE as usize * Self::SIZE as usize * Self::HEIGHT as usize],
        }
    }
}
