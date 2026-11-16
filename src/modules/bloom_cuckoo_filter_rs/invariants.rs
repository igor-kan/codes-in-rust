use crate::errors::CuckooError;
use crate::types::CuckooFilter;

pub fn verify_cuckoo_involution(cf: &CuckooFilter) -> Result<(), CuckooError> {
    for b in 0..cf.num_buckets {
        for &fp in &cf.buckets[b] {
            if fp != 0 {
                let alt = cf.alt_index(b, fp);
                if cf.alt_index(alt, fp) != b {
                    return Err(CuckooError::InvariantViolation("alternate bucket involution broken"));
                }
            }
        }
    }
    Ok(())
}
