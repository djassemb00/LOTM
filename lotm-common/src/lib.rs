//! # LOTM Common
//!
//! Common types and utilities used across all LOTM crates.

use serde::{Deserialize, Serialize};

/// 2D vector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// 3D vector
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

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
