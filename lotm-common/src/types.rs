//! Core types for LOTM game world.

use serde::{Deserialize, Serialize};
use vek::*;

/// World position (3D)
pub type WPos = Vec3<i32>;

/// Local position within a chunk
pub type LPos = Vec3<i32>;

/// Chunk position in the world grid
pub type CPos = Vec2<i32>;

/// 2D position
pub type Pos2D = Vec2<i32>;

/// Color in RGB
pub type Color = Rgb<u8>;

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
    Cobblestone,
    Glass,
    Iron,
    Gold,
    Crystal,
    Corruption,
    // LOTM specific blocks
    FogBarrier,
    BeyonderAltar,
    PotionIngredient,
    SealedArtifact,
}

impl BlockKind {
    pub fn is_solid(&self) -> bool {
        !matches!(self, Self::Air | Self::Water)
    }

    pub fn is_transparent(&self) -> bool {
        matches!(self, Self::Air | Self::Water | Self::Glass | Self::FogBarrier)
    }
}

/// Chunk data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub position: CPos,
    pub blocks: Vec<BlockKind>,
    pub entities: Vec<u64>,
}

impl Chunk {
    pub const SIZE: u32 = 32;
    pub const HEIGHT: u32 = 256;
    pub const TOTAL_BLOCKS: usize = Self::SIZE as usize * Self::SIZE as usize * Self::HEIGHT as usize;

    pub fn new(position: CPos) -> Self {
        Self {
            position,
            blocks: vec![BlockKind::Air; Self::TOTAL_BLOCKS],
            entities: Vec::new(),
        }
    }

    pub fn get_block(&self, local_pos: LPos) -> Option<BlockKind> {
        if self.is_valid_pos(local_pos) {
            let idx = self.pos_to_index(local_pos);
            self.blocks.get(idx).copied()
        } else {
            None
        }
    }

    pub fn set_block(&mut self, local_pos: LPos, kind: BlockKind) -> bool {
        if self.is_valid_pos(local_pos) {
            let idx = self.pos_to_index(local_pos);
            self.blocks[idx] = kind;
            true
        } else {
            false
        }
    }

    fn is_valid_pos(&self, pos: LPos) -> bool {
        pos.x >= 0
            && pos.x < Self::SIZE as i32
            && pos.y >= 0
            && pos.y < Self::SIZE as i32
            && pos.z >= 0
            && pos.z < Self::HEIGHT as i32
    }

    fn pos_to_index(&self, pos: LPos) -> usize {
        (pos.z as usize * Self::SIZE as usize + pos.y as usize) * Self::SIZE as usize
            + pos.x as usize
    }
}
