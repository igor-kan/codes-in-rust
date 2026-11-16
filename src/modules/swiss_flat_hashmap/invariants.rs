use crate::errors::SwissMapError;
use crate::interface::FlatDict;
use crate::types::SwissFlatMap;
use std::collections::HashMap;

pub fn verify_map_oracle(m: &SwissFlatMap, oracle: &HashMap<u64, i64>) -> Result<(), SwissMapError> {
    if m.len != oracle.len() {
        return Err(SwissMapError::InvariantViolation("length divergence"));
    }
    for (&k, &v) in oracle {
        if m.get(k) != Some(v) {
            return Err(SwissMapError::InvariantViolation("missing or mismatched key"));
        }
    }
    Ok(())
}
