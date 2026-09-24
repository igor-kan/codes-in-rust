//! Tests for importance_sampling
#[path = "../src/importance_sampling.rs"]
mod importance_sampling;
use importance_sampling::*;

#[test]
fn test_importance_sampling_execution() {
    assert_eq!(importance_weight(0.4, 0.2), 2.0);
}
