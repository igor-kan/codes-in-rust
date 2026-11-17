use crate::core::PairingHeap;
use crate::interface::IndexedMinHeap;

pub fn fast_heapsort(xs: &[i64]) -> Vec<i64> {
    let mut h = PairingHeap::new();
    for (i, &k) in xs.iter().enumerate() { h.push(k, i); }
    let mut out = Vec::with_capacity(xs.len());
    while let Some((k, _)) = h.pop_min() { out.push(k); }
    out
}
