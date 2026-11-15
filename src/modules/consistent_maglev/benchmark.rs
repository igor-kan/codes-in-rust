#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::build_maglev;
use std::time::Instant;

fn main() {
    let r = build_maglev(&["s0", "s1", "s2", "s3", "s4"], 65537).unwrap();
    let t0 = Instant::now();
    let mut sum = 0usize;
    for k in 0..50000u64 {
        sum += r.route_key(k);
    }
    println!("consistent_maglev ns={} sum={}", t0.elapsed().as_nanos() / 50000, sum);
}
