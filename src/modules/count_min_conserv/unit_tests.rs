#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use errors::SketchError;
use interface::{build_sketch, FrequencySketch};
use invariants::verify_no_underestimation;
use std::collections::HashMap;
use types::SketchConfig;
use validation::validate_sketch_config;

#[test]
fn test_conservative_update_and_merge() {
    let cfg = SketchConfig { width: 128, depth: 5, seed: 42 };
    assert_eq!(validate_sketch_config(&cfg), Ok(()));
    let mut s1 = build_sketch(&cfg).unwrap();
    let mut s2 = build_sketch(&cfg).unwrap();
    let mut exact = HashMap::new();
    for i in 0..200u64 {
        let k = i % 25;
        if i % 2 == 0 { s1.update_conservative(k, 1); } else { s2.update_conservative(k, 1); }
        *exact.entry(k).or_insert(0) += 1;
    }
    assert_eq!(s1.merge_sketch(&s2), Ok(()));
    assert_eq!(verify_no_underestimation(&s1, &exact), Ok(()));
    assert!(build_sketch(&SketchConfig { width: 0, depth: 3, seed: 1 }) == Err(SketchError::InvalidDimension("width and depth must be > 0")));
}
