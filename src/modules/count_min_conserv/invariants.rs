use crate::errors::SketchError;
use crate::interface::FrequencySketch;
use crate::types::ConservativeSketch;
use std::collections::HashMap;

pub fn verify_no_underestimation(sketch: &ConservativeSketch, exact: &HashMap<u64, u64>) -> Result<(), SketchError> {
    for (&k, &true_cnt) in exact {
        if sketch.estimate(k) < true_cnt {
            return Err(SketchError::InvariantViolation("count-min estimate underestimated true count"));
        }
    }
    Ok(())
}
