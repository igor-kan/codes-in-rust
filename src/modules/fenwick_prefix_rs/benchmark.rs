#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::lower_bound_prefix;
use interface::from_slice_linear;
use std::time::Instant;

fn main() {
    let mut bit = from_slice_linear(&vec![2i64; 4096]);
    let t0 = Instant::now();
    let mut acc = 0usize;
    for i in 0..5000usize {
        bit.add(i % 4096, 1);
        acc += lower_bound_prefix(&bit, 3000).unwrap_or(0);
    }
    println!("fenwick_prefix_rs ns={} acc={}", t0.elapsed().as_nanos() / 5000, acc);
}
