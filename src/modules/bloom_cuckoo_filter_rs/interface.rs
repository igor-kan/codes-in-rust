use crate::errors::CuckooError;
use crate::types::CuckooFilter;

pub fn make_cuckoo_filter(capacity_hint: usize, seed: u64) -> Result<CuckooFilter, CuckooError> {
    if capacity_hint < 16 {
        return Err(CuckooError::InvalidInput("capacity_hint must be >= 16"));
    }
    let nb = (capacity_hint / 4).next_power_of_two().max(8);
    Ok(CuckooFilter {
        num_buckets: nb,
        buckets: vec![[0u16; 4]; nb],
        count: 0,
        seed,
    })
}
