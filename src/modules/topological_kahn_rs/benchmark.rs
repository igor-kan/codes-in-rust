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
    let edges: Vec<(usize, usize)> = (0..127).map(|i| (i, i + 1)).collect();
    let t0 = Instant::now();
    for _it in 0..200u64 {
        if let types::TopoOutcome::Ordered(v) = core::kahn_min_heap(128, &edges) { sink = sink.wrapping_add(v.len() as u64); }
    }
    let ns = t0.elapsed().as_nanos() as f64 / 200.0;
    println!("topological_kahn_rs ns={:.1} sink={}", ns, sink);
}
