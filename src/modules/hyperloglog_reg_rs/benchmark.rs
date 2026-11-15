#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::observe_slice;
use interface::{new_hll, CardinalityEstimator};
use std::time::Instant;

fn main() {
    let mut h = new_hll(12).unwrap();
    let data: Vec<u64> = (0..4096u64).collect();
    let t0 = Instant::now();
    for _ in 0..100 {
        observe_slice(&mut h, &data);
    }
    println!("hyperloglog_reg_rs ns={} est={:.1}", t0.elapsed().as_nanos() / 100, h.estimate());
}
