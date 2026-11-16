use crate::errors::SwissMapError;
use crate::types::SwissFlatMap;

pub fn validate_map(m: &SwissFlatMap) -> Result<(), SwissMapError> {
    if !m.ctrl.len().is_power_of_two() || m.ctrl.len() != m.slots.len() {
        return Err(SwissMapError::InvariantViolation("capacity mismatch"));
    }
    if m.len + m.tombstones >= m.ctrl.len() {
        return Err(SwissMapError::CapacityFull);
    }
    Ok(())
}
