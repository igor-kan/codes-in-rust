#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::batch_update_unit;
use interface::{build_sketch, FrequencySketch};
use std::time::Instant;
use types::SketchConfig;

fn main() {
    let cfg = SketchConfig { width: 1024, depth: 4, seed: 7 };
    let mut s = build_sketch(&cfg).unwrap();
    let keys: Vec<u64> = (0..2048u64).collect();
    let t0 = Instant::now();
    for _ in 0..200 {
        batch_update_unit(&mut s, &keys);
    }
    let elapsed = t0.elapsed().as_nanos();
    println!("count_min_conserv ns={} est0={}", elapsed / 200, s.estimate(0));
}
