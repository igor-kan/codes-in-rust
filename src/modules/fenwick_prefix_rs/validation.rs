use crate::errors::FenwickError;
use crate::types::FenwickTree;

pub fn validate_non_negative_weights(bit: &FenwickTree) -> Result<(), FenwickError> {
    if bit.raw.iter().any(|&v| v < 0) {
        return Err(FenwickError::InvariantViolation("negative weight forbids monotone lifting"));
    }
    Ok(())
}
