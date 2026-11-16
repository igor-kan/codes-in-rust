use crate::types::CuckooFilter;

pub fn batch_contains(cf: &CuckooFilter, keys: &[u64]) -> Vec<bool> {
    keys.iter().map(|&k| cf.contains(k)).collect()
}
