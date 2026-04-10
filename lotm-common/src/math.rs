//! Math utilities for LOTM.

use vek::*;

/// Convert world position to chunk position
pub fn wpos_to_cpos(wpos: Vec2<i32>, chunk_size: u32) -> Vec2<i32> {
    wpos.map(|e| e.div_euclid(chunk_size as i32))
}

/// Convert chunk position to world position
pub fn cpos_to_wpos(cpos: Vec2<i32>, chunk_size: u32) -> Vec2<i32> {
    cpos.map(|e| e * chunk_size as i32)
}

/// Linear interpolation
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
