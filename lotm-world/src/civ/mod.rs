//! Civilization generation - cities, settlements, trade routes.
//!
//! LOTM cities:
//! - Tingen City (starting area)
//! - Backlund (capital, massive industrial city)
//! - Bayam (coastal trading port)
//! - Intis Republic cities
//! - Feynapotter (Southern Continent)

use lotm_common::vek::*;

/// Civilization generator
pub struct CivGen;

impl CivGen {
    /// Generate civilizations
    pub fn generate(
        seed: u32,
        sim: &mut crate::sim::WorldSim,
        index: &mut crate::Index,
        progress_cb: &(dyn Fn(f64) + Send + Sync),
    ) -> Vec<City> {
        let cities = Self::place_cities(seed, sim, index);
        progress_cb(1.0);
        cities
    }

    fn place_cities(
        seed: u32,
        sim: &mut crate::sim::WorldSim,
        index: &mut crate::Index,
    ) -> Vec<City> {
        let mut cities = Vec::new();

        // Place major cities at specific locations
        cities.push(City::new("Tingen", Vec2::new(100, 100), CityKind::StartingCity));
        cities.push(City::new("Backlund", Vec2::new(512, 512), CityKind::Capital));
        cities.push(City::new("Bayam", Vec2::new(200, 800), CityKind::CoastalPort));

        // Mark city chunks in simulation
        for city in &cities {
            Self::mark_city_chunk(sim, city);
        }

        cities
    }

    fn mark_city_chunk(sim: &mut crate::sim::WorldSim, city: &City) {
        let chunk_size = lotm_common::types::Chunk::SIZE as i32;
        let city_chunk = city.position.map(|e| e.div_euclid(chunk_size));

        // Mark surrounding chunks as city
        for dy in -3..=3 {
            for dx in -3..=3 {
                let chunk_pos = city_chunk + Vec2::new(dx, dy);
                if let Some(chunk) = sim.chunks.get_mut(chunk_pos.y as usize * 1024 + chunk_pos.x as usize) {
                    let dist = Vec2::new(dx, dy).magnitude();
                    chunk.city_factor = chunk.city_factor.max(1.0 - dist / 4.0);
                }
            }
        }
    }
}

/// City data
#[derive(Debug, Clone)]
pub struct City {
    pub name: &'static str,
    pub position: Vec2<i32>,
    pub kind: CityKind,
}

impl City {
    pub fn new(name: &'static str, position: Vec2<i32>, kind: CityKind) -> Self {
        Self { name, position, kind }
    }
}

/// City types
#[derive(Debug, Clone, Copy)]
pub enum CityKind {
    StartingCity,
    Capital,
    CoastalPort,
    IndustrialCenter,
    ReligiousCenter,
}
