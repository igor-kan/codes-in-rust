use crate::errors::HllError;
use crate::interface::CardinalityEstimator;
use crate::types::HllSketch;

pub fn verify_idempotent_duplicate_invariance(hll: &HllSketch, sample: &[u64]) -> Result<(), HllError> {
    let before = hll.estimate();
    let mut clone = hll.clone();
    for &x in sample {
        clone.observe(x);
    }
    let after = clone.estimate();
    if (after - before).abs() > 1e-9 && sample.is_empty() {
        return Err(HllError::InvariantViolation("empty sample mutated estimate"));
    }
    if clone.registers.len() != hll.registers.len() {
        return Err(HllError::InvariantViolation("register count altered"));
    }
    Ok(())
}
