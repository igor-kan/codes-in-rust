use crate::errors::HllError;
use crate::types::HllSketch;

pub fn validate_hll(hll: &HllSketch) -> Result<(), HllError> {
    if !(4..=16).contains(&hll.precision) || hll.registers.len() != (1usize << hll.precision) {
        return Err(HllError::InvariantViolation("register length != 2^p"));
    }
    Ok(())
}
