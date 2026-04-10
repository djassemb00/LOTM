//! Utility functions for world generation.

use lotm_common::vek::*;

/// Fast hash for seeding
pub fn fast_hash(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = seed.wrapping_add(0x9e3779b9);
    h = h.wrapping_add((x as u32).wrapping_mul(73856093));
    h = h.wrapping_add((y as u32).wrapping_mul(19349663));
    h = h ^ (h >> 16);
    h = h.wrapping_mul(0x85ebca6b);
    h = h ^ (h >> 13);
    h = h.wrapping_mul(0xc2b2ae35);
    h ^ (h >> 16)
}

/// Linear interpolation
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Bilinear interpolation
pub fn bilinear(v00: f32, v10: f32, v01: f32, v11: f32, t: Vec2<f32>) -> f32 {
    let v0 = lerp(v00, v10, t.x);
    let v1 = lerp(v01, v11, t.x);
    lerp(v0, v1, t.y)
}

/// Smoothstep
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
