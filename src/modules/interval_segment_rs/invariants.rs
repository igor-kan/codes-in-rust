use crate::errors::IntervalError;
use crate::types::Interval;

pub fn verify_canonical_disjoint(coalesced: &[Interval]) -> Result<(), IntervalError> {
    for w in coalesced.windows(2) {
        if w[0].start >= w[0].end || w[0].end >= w[1].start {
            return Err(IntervalError::InvariantViolation("intervals not strictly sorted and disjoint"));
        }
    }
    Ok(())
}
