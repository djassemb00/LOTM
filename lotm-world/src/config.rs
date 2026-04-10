//! World generation configuration.

/// Sea level altitude
pub const SEA_LEVEL: f32 = 140.0;

/// Mountain scale
pub const MOUNTAIN_SCALE: f32 = 2048.0;

/// Temperature thresholds for biomes
pub const SNOW_TEMP: f32 = -0.8;
pub const TEMPERATE_TEMP: f32 = -0.4;
pub const TROPICAL_TEMP: f32 = 0.4;

/// Corruption thresholds
pub const LOW_CORRUPTION: f32 = 0.3;
pub const MEDIUM_CORRUPTION: f32 = 0.5;
pub const HIGH_CORRUPTION: f32 = 0.7;

/// World configuration
pub struct Config {
    pub sea_level: f32,
    pub mountain_scale: f32,
    pub snow_temp: f32,
    pub temperate_temp: f32,
    pub tropical_temp: f32,
}

pub const CONFIG: Config = Config {
    sea_level: SEA_LEVEL,
    mountain_scale: MOUNTAIN_SCALE,
    snow_temp: SNOW_TEMP,
    temperate_temp: TEMPERATE_TEMP,
    tropical_temp: TROPICAL_TEMP,
};
