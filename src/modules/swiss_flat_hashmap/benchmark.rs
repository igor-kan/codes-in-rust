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
    let mut m = interface::with_capacity(512, 99).unwrap(); for k in 0..256u64 { interface::FlatDict::insert(&mut m, k, k as i64); }
    let t0 = Instant::now();
    for _it in 0..200u64 {
        for k in 0..128u64 { sink = sink.wrapping_add(interface::FlatDict::get(&m, (k + _it) & 255).unwrap_or(0) as u64); }
    }
    let ns = t0.elapsed().as_nanos() as f64 / 200.0;
    println!("swiss_flat_hashmap ns={:.1} sink={}", ns, sink);
}
