//! Column sampling - samples a vertical column of the world.

use lotm_common::vek::*;

/// Sample of a world column
#[derive(Debug, Clone)]
pub struct ColumnSample<'a> {
    /// Surface altitude
    pub alt: f32,
    /// Basement altitude
    pub basement: f32,
    /// Chaos factor
    pub chaos: f32,
    /// Water level
    pub water_level: f32,
    /// Surface color
    pub surface_color: Vec3<f32>,
    /// Sub-surface color
    pub sub_surface_color: Vec3<f32>,
    /// Stone color
    pub stone_col: Vec3<u8>,
    /// Snow cover
    pub snow_cover: bool,
    /// Cliff offset
    pub cliff_offset: f32,
    /// Cliff height
    pub cliff_height: f32,
    /// Ice depth
    pub ice_depth: f32,
    /// Temperature
    pub temp: f32,
    /// Humidity
    pub humidity: f32,
    /// Corruption level
    pub corruption: f32,
    /// Marble pattern (mid)
    pub marble_mid: f32,
    /// Reference to simulation
    _marker: std::marker::PhantomData<&'a ()>,
}

/// Column generator
pub struct ColumnGen<'a> {
    pub sim: &'a crate::sim::WorldSim,
}

impl<'a> ColumnGen<'a> {
    pub fn new(sim: &'a crate::sim::WorldSim) -> Self {
        Self { sim }
    }

    /// Sample a column at world position
    pub fn get(
        &self,
        (wpos, index, _calendar): (Vec2<i32>, crate::IndexRef<'a>, Option<()>),
    ) -> Option<ColumnSample<'a>> {
        let chunk_pos = wpos.map(|e| e.div_euclid(lotm_common::types::Chunk::SIZE as i32));

        let sim_chunk = self.sim.get(chunk_pos)?;

        // Interpolate values
        let alt = self.sim.get_interpolated(wpos, |c| c.alt)?;
        let basement = self.sim.get_interpolated(wpos, |c| c.basement)?;
        let chaos = self.sim.get_interpolated(wpos, |c| c.chaos)?;
        let temp = self.sim.get_interpolated(wpos, |c| c.temp)?;
        let humidity = self.sim.get_interpolated(wpos, |c| c.humidity)?;
        let corruption = self.sim.get_interpolated(wpos, |c| c.corruption)?;

        // Calculate colors based on biome
        let surface_color = calculate_surface_color(sim_chunk, temp, humidity);
        let sub_surface_color = calculate_sub_surface_color(sim_chunk);

        Some(ColumnSample {
            alt,
            basement,
            chaos,
            water_level: sim_chunk.water_alt,
            surface_color,
            sub_surface_color,
            stone_col: Vec3::new(100, 100, 100),
            snow_cover: temp < crate::config::SNOW_TEMP,
            cliff_offset: chaos * 16.0,
            cliff_height: 8.0 + chaos * 16.0,
            ice_depth: if temp < 0.0 { 4.0 } else { 0.0 },
            temp,
            humidity,
            corruption,
            marble_mid: (noise::Fbm::new(index.seed + 42).get([wpos.x as f64 / 12.0, wpos.y as f64 / 12.0]) * 0.75 + 1.0) * 0.5,
            _marker: std::marker::PhantomData,
        })
    }
}

fn calculate_surface_color(
    chunk: &crate::sim::SimChunk,
    temp: f32,
    humidity: f32,
) -> Vec3<f32> {
    use crate::sim::BiomeKind;

    match chunk.biome {
        BiomeKind::Corrupted => Vec3::new(0.3, 0.1, 0.4),
        BiomeKind::FoggyIndustrial => Vec3::new(0.4, 0.4, 0.35),
        BiomeKind::SafeCity => Vec3::new(0.5, 0.45, 0.4),
        BiomeKind::Coastal => Vec3::new(0.6, 0.55, 0.4),
        BiomeKind::Highland => Vec3::new(0.45, 0.5, 0.45),
        BiomeKind::ForsakenLand => Vec3::new(0.35, 0.3, 0.25),
        BiomeKind::SacredLand => Vec3::new(0.55, 0.5, 0.6),
        BiomeKind::Normal => {
            // Gradient based on temp and humidity
            let green = 0.3 + humidity * 0.3;
            let brown = 0.4 - humidity * 0.1;
            Vec3::new(brown, green, brown * 0.5)
        },
    }
}

fn calculate_sub_surface_color(chunk: &crate::sim::SimChunk) -> Vec3<f32> {
    let base = 0.3 + chunk.rockiness * 0.2;
    Vec3::new(base, base * 0.9, base * 0.8)
}
