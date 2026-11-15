use crate::errors::IntervalError;
use crate::types::Interval;

pub fn validate_non_empty(raw: &[Interval]) -> Result<(), IntervalError> {
    if raw.is_empty() {
        return Err(IntervalError::EmptyInput);
    }
    Ok(())
}
