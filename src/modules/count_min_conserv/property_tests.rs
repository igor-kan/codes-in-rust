#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::{build_sketch, FrequencySketch};
use invariants::verify_no_underestimation;
use std::collections::HashMap;
use types::SketchConfig;

#[test]
fn prop_conservative_never_underestimates_zipf() {
    let cfg = SketchConfig { width: 64, depth: 4, seed: 1337 };
    let mut s = build_sketch(&cfg).unwrap();
    let mut exact = HashMap::new();
    for step in 0..1000u64 {
        let key = (step.wrapping_mul(2654435761)) % 97;
        s.update_conservative(key, 3);
        *exact.entry(key).or_insert(0) += 3;
    }
    assert_eq!(verify_no_underestimation(&s, &exact), Ok(()));
}
