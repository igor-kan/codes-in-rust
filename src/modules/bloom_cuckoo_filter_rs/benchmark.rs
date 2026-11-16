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
    let mut cf = interface::make_cuckoo_filter(2048, 13).unwrap(); for k in 0..1024u64 { cf.insert(k).unwrap(); }
    let t0 = Instant::now();
    for _it in 0..200u64 {
        for q in 0..256u64 { if cf.contains(q + _it) { sink += 1; } }
    }
    let ns = t0.elapsed().as_nanos() as f64 / 200.0;
    println!("bloom_cuckoo_filter_rs ns={:.1} sink={}", ns, sink);
}
