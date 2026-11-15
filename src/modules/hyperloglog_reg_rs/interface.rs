use crate::errors::HllError;
use crate::types::HllSketch;

pub trait CardinalityEstimator {
    fn observe(&mut self, item: u64);
    fn estimate(&self) -> f64;
    fn merge(&mut self, other: &Self) -> Result<(), HllError>;
}

pub fn new_hll(precision: u8) -> Result<HllSketch, HllError> {
    if !(4..=16).contains(&precision) {
        return Err(HllError::InvalidDimension("precision p must be in [4, 16]"));
    }
    Ok(HllSketch {
        precision,
        registers: vec![0u8; 1usize << precision],
    })
}
