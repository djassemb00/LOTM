//! # LOTM Common
//!
//! Common types used across all LOTM crates.

/// A chunk position in the world grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Block kinds in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKind {
    Air,
    Stone,
    Earth,
    Grass,
    Water,
}

impl BlockKind {
    pub fn is_solid(&self) -> bool {
        !matches!(self, Self::Air | Self::Water)
    }
}
