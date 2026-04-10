//! Index - shared data for world generation.

use lotm_common::vek::*;
use std::sync::Arc;

/// World index - holds shared generation data
pub struct Index {
    pub seed: u32,
    pub noise: Noise,
}

/// Noise generators for various purposes
pub struct Noise {
    pub cave_nz: noise::SuperSimplex,
    pub scatter_nz: noise::SuperSimplex,
    pub structure_nz: noise::SuperSimplex,
}

impl Index {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            noise: Noise::new(seed),
        }
    }
}

impl Noise {
    fn new(seed: u32) -> Self {
        Self {
            cave_nz: noise::SuperSimplex::new(seed),
            scatter_nz: noise::SuperSimplex::new(seed + 1),
            structure_nz: noise::SuperSimplex::new(seed + 2),
        }
    }
}

/// Owned reference to index
#[derive(Clone)]
pub struct IndexOwned {
    index: Arc<Index>,
}

impl IndexOwned {
    pub fn new(index: Index) -> Self {
        Self {
            index: Arc::new(index),
        }
    }

    pub fn as_ref(&self) -> IndexRef {
        IndexRef {
            index: &self.index,
        }
    }
}

impl std::ops::Deref for IndexOwned {
    type Target = Index;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

/// Shared reference to index
#[derive(Clone, Copy)]
pub struct IndexRef<'a> {
    pub index: &'a Index,
}

impl std::ops::Deref for IndexRef<'_> {
    type Target = Index;

    fn deref(&self) -> &Self::Target {
        self.index
    }
}
