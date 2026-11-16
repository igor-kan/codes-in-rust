use crate::errors::CuckooError;
use crate::types::CuckooFilter;

pub fn load_factor(cf: &CuckooFilter) -> Result<f64, CuckooError> {
    if !cf.num_buckets.is_power_of_two() {
        return Err(CuckooError::InvariantViolation("bucket count not power of two"));
    }
    Ok(cf.count as f64 / (cf.num_buckets * 4) as f64)
}
