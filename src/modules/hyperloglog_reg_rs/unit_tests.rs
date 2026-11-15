#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::{new_hll, CardinalityEstimator};
use validation::validate_hll;

#[test]
fn test_hll_accuracy_and_duplicates() {
    let mut hll = new_hll(12).unwrap();
    assert_eq!(validate_hll(&hll), Ok(()));
    for i in 0..5000u64 {
        hll.observe(i);
        hll.observe(i);
    }
    let est = hll.estimate();
    let rel_err = (est - 5000.0).abs() / 5000.0;
    assert!(rel_err < 0.08, "relative error too high: {}", rel_err);
}
