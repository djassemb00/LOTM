//! Block generation from column samples.

use lotm_common::types::BlockKind;
use lotm_common::vek::*;

/// Get block kind at position in column
pub fn get_block_at(col: &crate::column::ColumnSample, z: i32, index: &crate::Index) -> BlockKind {
    let alt = col.alt as i32;
    let basement = col.basement as i32;

    if z > alt {
        // Above surface
        if z < col.water_level as i32 {
            BlockKind::Water
        } else if col.snow_cover && z <= alt + 2 {
            BlockKind::Grass // Snow on grass
        } else {
            BlockKind::Air
        }
    } else if z == alt {
        // Surface
        if col.water_level as i32 > alt {
            BlockKind::Earth // Underwater surface
        } else if col.corruption > 0.7 {
            BlockKind::Corruption
        } else {
            BlockKind::Grass
        }
    } else if z > alt - 4 {
        // Just below surface
        if col.corruption > 0.5 {
            BlockKind::Corruption
        } else {
            BlockKind::Earth
        }
    } else if z > basement {
        // Underground
        if col.marble_mid > 0.7 {
            BlockKind::Stone
        } else if col.corruption > 0.6 {
            BlockKind::Corruption
        } else {
            BlockKind::Stone
        }
    } else {
        // Deep underground
        BlockKind::Stone
    }
}
