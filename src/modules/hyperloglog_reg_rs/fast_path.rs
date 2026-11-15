use crate::interface::CardinalityEstimator;
use crate::types::HllSketch;

pub fn observe_slice(hll: &mut HllSketch, items: &[u64]) {
    for &x in items {
        hll.observe(x);
    }
}
