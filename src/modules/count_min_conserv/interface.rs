use crate::errors::SketchError;
use crate::types::{ConservativeSketch, SketchConfig};

pub trait FrequencySketch {
    fn update_conservative(&mut self, key: u64, count: u64);
    fn estimate(&self, key: u64) -> u64;
    fn merge_sketch(&mut self, other: &Self) -> Result<(), SketchError>;
}

pub fn build_sketch(cfg: &SketchConfig) -> Result<ConservativeSketch, SketchError> {
    if cfg.width == 0 || cfg.depth == 0 {
        return Err(SketchError::InvalidDimension("width and depth must be > 0"));
    }
    Ok(ConservativeSketch {
        width: cfg.width,
        depth: cfg.depth,
        seed: cfg.seed,
        rows: vec![vec![0u64; cfg.width]; cfg.depth],
        total_mass: 0,
    })
}
