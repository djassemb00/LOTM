//! Error types for LOTM.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LotmError {
    #[error("World generation error: {0}")]
    WorldGen(String),

    #[error("Asset loading error: {0}")]
    AssetLoad(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, LotmError>;
