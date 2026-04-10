//! Fog and Lovecraftian atmosphere system.
//!
//! Handles:
//! - Volumetric fog rendering
//! - Corruption fog visualization
//! - Atmospheric effects
//! - Lighting in foggy conditions

use lotm_common::vek::*;

/// Fog configuration
#[derive(Debug, Clone)]
pub struct FogConfig {
    /// Fog density (0.0 = clear, 1.0 = opaque)
    pub density: f32,
    /// Fog color
    pub color: Vec3<f32>,
    /// Fog height (where fog starts)
    pub height: f32,
    /// Fog falloff (how quickly it thins with height)
    pub falloff: f32,
    /// Corruption influence (adds eerie effects)
    pub corruption: f32,
}

impl FogConfig {
    /// Clear weather
    pub fn clear() -> Self {
        Self {
            density: 0.1,
            color: Vec3::new(0.7, 0.75, 0.8),
            height: 100.0,
            falloff: 0.02,
            corruption: 0.0,
        }
    }

    /// Foggy industrial weather
    pub fn foggy() -> Self {
        Self {
            density: 0.6,
            color: Vec3::new(0.4, 0.4, 0.35),
            height: 20.0,
            falloff: 0.01,
            corruption: 0.2,
        }
    }

    /// Heavy corruption fog
    pub fn corrupted() -> Self {
        Self {
            density: 0.9,
            color: Vec3::new(0.3, 0.1, 0.4),
            height: 5.0,
            falloff: 0.005,
            corruption: 1.0,
        }
    }

    /// Interpolate between fog configs
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            density: lotm_common::math::lerp(self.density, other.density, t),
            color: self.color + (other.color - self.color) * t,
            height: lotm_common::math::lerp(self.height, other.height, t),
            falloff: lotm_common::math::lerp(self.falloff, other.falloff, t),
            corruption: lotm_common::math::lerp(self.corruption, other.corruption, t),
        }
    }
}

/// Atmospheric effects manager
pub struct Atmosphere {
    pub fog: FogConfig,
    pub target_fog: FogConfig,
    pub interpolation_speed: f32,
    pub wind_direction: Vec2<f32>,
    pub wind_speed: f32,
    pub time_of_day: f32,
}

impl Atmosphere {
    pub fn new() -> Self {
        Self {
            fog: FogConfig::clear(),
            target_fog: FogConfig::clear(),
            interpolation_speed: 0.1,
            wind_direction: Vec2::new(1.0, 0.0),
            wind_speed: 1.0,
            time_of_day: 0.5,
        }
    }

    /// Update atmosphere
    pub fn update(&mut self, delta_time: f32) {
        // Interpolate fog towards target
        let t = 1.0 - (-self.interpolation_speed * delta_time).exp();

        self.fog.density = lotm_common::math::lerp(self.fog.density, self.target_fog.density, t);
        self.fog.color = self.fog.color + (self.target_fog.color - self.fog.color) * t;
        self.fog.height = lotm_common::math::lerp(self.fog.height, self.target_fog.height, t);
        self.fog.falloff = lotm_common::math::lerp(self.fog.falloff, self.target_fog.falloff, t);
        self.fog.corruption = lotm_common::math::lerp(self.fog.corruption, self.target_fog.corruption, t);
    }

    /// Set target fog based on biome/corruption
    pub fn set_biome(&mut self, corruption: f32, is_industrial: bool) {
        self.target_fog = if corruption > 0.7 {
            FogConfig::corrupted()
        } else if is_industrial || corruption > 0.3 {
            FogConfig::foggy()
        } else {
            FogConfig::clear()
        };
    }

    /// Calculate fog density at position
    pub fn fog_at_position(&self, position: Vec3<f32>) -> f32 {
        let height_factor = (-self.fog.falloff * (position.z - self.fog.height)).exp();
        self.fog.density * height_factor.min(1.0)
    }

    /// Get visibility distance
    pub fn visibility_distance(&self) -> f32 {
        if self.fog.density < 0.01 {
            1000.0
        } else {
            10.0 / self.fog.density
        }
    }
}

impl Default for Atmosphere {
    fn default() -> Self {
        Self::new()
    }
}
