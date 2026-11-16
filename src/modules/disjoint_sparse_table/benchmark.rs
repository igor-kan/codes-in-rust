#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use std::time::Instant;

fn main() {
    let mut sink: u64 = 0;
    let a: Vec<u64> = (1..=512).collect(); let dst = interface::build_mod_mul_table(&a, 1_000_000_007).unwrap();
    let t0 = Instant::now();
    for _it in 0..200u64 {
        for q in 0..100usize { sink = sink.wrapping_add(dst.query_inclusive(q, q + 300)); }
    }
    let ns = t0.elapsed().as_nanos() as f64 / 200.0;
    println!("disjoint_sparse_table ns={:.1} sink={}", ns, sink);
}
