//! World simulation - base terrain generation.
//!
//! This module handles the foundational terrain generation including:
//! - Height maps using Perlin/Simplex noise
//! - Temperature and humidity simulation
//! - River and water flow simulation
//! - Biome determination

use lotm_common::vek::*;
use noise::{Fbm, HybridMulti, Perlin, RidgedMulti, SuperSimplex, Billow, NoiseFn};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// Generation context - holds all noise generators
pub struct GenCtx {
    /// Altitude noise
    pub alt_nz: HybridMulti<Perlin>,
    /// Chaos/variation noise
    pub chaos_nz: RidgedMulti<Perlin>,
    /// Temperature noise
    pub temp_nz: Fbm<Perlin>,
    /// Humidity noise
    pub humid_nz: Billow<Perlin>,
    /// Small detail noise
    pub small_nz: SuperSimplex,
    /// River seed noise
    pub river_nz: SuperSimplex,
    /// Corruption/fog noise
    pub corruption_nz: Fbm<Perlin>,
    /// City placement noise
    pub city_nz: SuperSimplex,
}

impl GenCtx {
    pub fn new(seed: u32) -> Self {
        Self {
            alt_nz: HybridMulti::new(seed),
            chaos_nz: RidgedMulti::new(seed + 1),
            temp_nz: Fbm::new(seed + 2).set_octaves(4),
            humid_nz: Billow::new(seed + 3).set_octaves(3),
            small_nz: SuperSimplex::new(seed + 4),
            river_nz: SuperSimplex::new(seed + 5),
            corruption_nz: Fbm::new(seed + 6).set_octaves(5),
            city_nz: SuperSimplex::new(seed + 7),
        }
    }
}

/// A single chunk in the simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimChunk {
    /// Base altitude
    pub alt: f32,
    /// Basement altitude (underground)
    pub basement: f32,
    /// Chaos/variation factor
    pub chaos: f32,
    /// Temperature (-1.0 to 1.0)
    pub temp: f32,
    /// Humidity (0.0 to 1.0)
    pub humidity: f32,
    /// Water altitude
    pub water_alt: f32,
    /// River data
    pub river: RiverData,
    /// Corruption level (0.0 to 1.0)
    pub corruption: f32,
    /// City proximity factor
    pub city_factor: f32,
    /// Biome kind
    pub biome: BiomeKind,
    /// Tree density
    pub tree_density: f32,
    /// Rockiness
    pub rockiness: f32,
}

impl SimChunk {
    pub fn empty() -> Self {
        Self {
            alt: 0.0,
            basement: 0.0,
            chaos: 0.0,
            temp: 0.0,
            humidity: 0.0,
            water_alt: 0.0,
            river: RiverData::default(),
            corruption: 0.0,
            city_factor: 0.0,
            biome: BiomeKind::default(),
            tree_density: 0.0,
            rockiness: 0.0,
        }
    }
}

/// River data for a chunk
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RiverData {
    pub is_river: bool,
    pub is_lake: bool,
    pub is_ocean: bool,
    pub velocity: Vec3<f32>,
    pub width: f32,
    pub depth: f32,
}

/// Biome kinds specific to LOTM universe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BiomeKind {
    #[default]
    Normal,
    /// Foggy industrial area
    FoggyIndustrial,
    /// Corrupted land
    Corrupted,
    /// Safe city area
    SafeCity,
    /// Coastal area
    Coastal,
    /// Highland
    Highland,
    /// Underground
    Underground,
    /// Forsaken land
    ForsakenLand,
    /// Sacred land
    SacredLand,
}

impl BiomeKind {
    pub fn from_conditions(temp: f32, humidity: f32, corruption: f32, city_factor: f32) -> Self {
        if corruption > 0.7 {
            BiomeKind::Corrupted
        } else if corruption > 0.4 {
            BiomeKind::FoggyIndustrial
        } else if city_factor > 0.6 {
            BiomeKind::SafeCity
        } else if corruption > 0.2 {
            BiomeKind::ForsakenLand
        } else if temp < -0.5 {
            BiomeKind::Highland
        } else if humidity > 0.7 {
            BiomeKind::Coastal
        } else {
            BiomeKind::Normal
        }
    }
}

/// World generation options
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorldOpts {
    /// Base 2 logarithm of world size in chunks (x)
    pub x_lg: u32,
    /// Base 2 logarithm of world size in chunks (y)
    pub y_lg: u32,
    /// Continent scale
    pub scale: f64,
    /// Erosion quality multiplier
    pub erosion_quality: f32,
}

impl Default for WorldOpts {
    fn default() -> Self {
        Self {
            x_lg: 10, // 1024 chunks
            y_lg: 10,
            scale: 2.0,
            erosion_quality: 1.0,
        }
    }
}

/// World simulation state
pub struct WorldSim {
    pub seed: u32,
    pub chunks: Vec<SimChunk>,
    pub map_size_lg: Vec2<u32>,
    pub gen_ctx: GenCtx,
    pub rng: ChaChaRng,
    pub max_height: f32,
}

impl WorldSim {
    /// Generate world simulation
    pub fn generate(
        seed: u32,
        opts: WorldOpts,
        _threadpool: &rayon::ThreadPool,
        progress_cb: &(dyn Fn(f64) + Send + Sync),
    ) -> Self {
        let gen_ctx = GenCtx::new(seed);
        let map_size_lg = Vec2::new(opts.x_lg, opts.y_lg);
        let total_chunks = (1 << opts.x_lg) * (1 << opts.y_lg);

        // Generate chunks in parallel
        let chunks: Vec<SimChunk> = (0..total_chunks)
            .into_par_iter()
            .enumerate()
            .map(|(idx, _)| {
                let x = idx % (1 << opts.x_lg);
                let y = idx / (1 << opts.x_lg);
                let chunk_pos = Vec2::new(x as i32, y as i32);

                Self::generate_chunk(chunk_pos, &gen_ctx, &opts)
            })
            .collect();

        let max_height = chunks.iter().map(|c| c.alt).fold(0.0f32, f32::max);

        progress_cb(1.0);

        Self {
            seed,
            chunks,
            map_size_lg,
            gen_ctx,
            rng: ChaChaRng::seed_from_u64(seed as u64),
            max_height,
        }
    }

    fn generate_chunk(chunk_pos: Vec2<i32>, ctx: &GenCtx, opts: &WorldOpts) -> SimChunk {
        let scale = opts.scale;
        let chunk_size = lotm_common::types::Chunk::SIZE as f64;

        // Sample noise at chunk center
        let pos = chunk_pos.map(|e| e as f64 * chunk_size + chunk_size / 2.0);

        // Altitude
        let alt = ctx.alt_nz.get((pos / (256.0 * scale)).into_array()) as f32;
        let alt = alt * 256.0 + 128.0; // Scale to reasonable heights

        // Basement (underground)
        let basement = ctx.alt_nz.get((pos / (512.0 * scale)).into_array()) as f32;
        let basement = alt * 0.3 + basement * 64.0;

        // Chaos/variation
        let chaos = ctx.chaos_nz.get((pos / (128.0 * scale)).into_array()) as f32;
        let chaos = (chaos * 0.5 + 0.5).powi(2);

        // Temperature
        let temp = ctx.temp_nz.get((pos / (1024.0 * scale)).into_array()) as f32;

        // Humidity
        let humidity = ctx.humid_nz.get((pos / (512.0 * scale)).into_array()) as f32;
        let humidity = humidity * 0.5 + 0.5;

        // Water level
        let water_alt = if alt < 140.0 { 140.0 } else { alt - 20.0 };

        // Corruption
        let corruption = ctx.corruption_nz.get((pos / (768.0 * scale)).into_array()) as f32;
        let corruption = (corruption * 0.5 + 0.5).powi(3);

        // City factor
        let city_factor = ctx.city_nz.get((pos / (2048.0 * scale)).into_array()) as f32;
        let city_factor = (city_factor * 0.5 + 0.5).powi(4);

        // Determine biome
        let biome = BiomeKind::from_conditions(temp, humidity, corruption, city_factor);

        // Tree density (lower in corrupted/city areas)
        let tree_density = if corruption > 0.3 || city_factor > 0.5 {
            0.0
        } else {
            (humidity * (1.0 - corruption)).max(0.0)
        };

        // Rockiness
        let rockiness = ctx.small_nz.get((pos / (64.0 * scale)).into_array()) as f32;
        let rockiness = (rockiness * 0.5 + 0.5).powi(2);

        SimChunk {
            alt,
            basement,
            chaos,
            temp,
            humidity,
            water_alt,
            river: RiverData::default(),
            corruption,
            city_factor,
            biome,
            tree_density,
            rockiness,
        }
    }

    /// Get chunk at position
    pub fn get(&self, chunk_pos: Vec2<i32>) -> Option<&SimChunk> {
        if self.is_valid_pos(chunk_pos) {
            let idx = self.pos_to_index(chunk_pos);
            self.chunks.get(idx)
        } else {
            None
        }
    }

    /// Interpolate values at world position
    pub fn get_interpolated<T>(
        &self,
        wpos: Vec2<i32>,
        extract: impl Fn(&SimChunk) -> T,
    ) -> Option<T>
    where
        T: std::ops::Add<Output = T>
            + std::ops::Mul<f32, Output = T>
            + Copy
            + Default,
    {
        let chunk_size = lotm_common::types::Chunk::SIZE as i32;
        let chunk_pos = wpos.map(|e| e.div_euclid(chunk_size));
        let local_pos = wpos.map(|e| e.rem_euclid(chunk_size));
        let t = local_pos.map(|e| e as f32 / chunk_size as f32);

        // Get 4 surrounding chunks
        let c00 = self.get(chunk_pos);
        let c10 = self.get(chunk_pos + Vec2::new(1, 0));
        let c01 = self.get(chunk_pos + Vec2::new(0, 1));
        let c11 = self.get(chunk_pos + Vec2::new(1, 1));

        if let (Some(c00), Some(c10), Some(c01), Some(c11)) = (c00, c10, c01, c11) {
            let v00 = extract(c00);
            let v10 = extract(c10);
            let v01 = extract(c01);
            let v11 = extract(c11);

            // Bilinear interpolation
            let v0 = v00 * (1.0 - t.x) + v10 * t.x;
            let v1 = v01 * (1.0 - t.x) + v11 * t.x;
            Some(v0 * (1.0 - t.y) + v1 * t.y)
        } else {
            None
        }
    }

    fn is_valid_pos(&self, chunk_pos: Vec2<i32>) -> bool {
        let size = self.map_size_lg.map(|e| 1 << e);
        chunk_pos.x >= 0 && chunk_pos.x < size.x as i32 && chunk_pos.y >= 0 && chunk_pos.y < size.y as i32
    }

    fn pos_to_index(&self, chunk_pos: Vec2<i32>) -> usize {
        let size_x = 1 << self.map_size_lg.x;
        (chunk_pos.y as usize * size_x) + chunk_pos.x as usize
    }
}
