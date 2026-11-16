use crate::interface::FlatDict;
use crate::types::SwissFlatMap;

pub fn batch_get(m: &SwissFlatMap, keys: &[u64]) -> Vec<Option<i64>> {
    let mut out = Vec::with_capacity(keys.len());
    for &k in keys {
        out.push(m.get(k));
    }
    out
}
