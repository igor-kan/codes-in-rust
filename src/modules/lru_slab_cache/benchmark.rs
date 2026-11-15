#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::create_lru;
use std::time::Instant;

fn main() {
    let mut lru = create_lru::<u64, u64>(256).unwrap();
    let t0 = Instant::now();
    let mut acc = 0u64;
    for i in 0..10000u64 {
        lru.put(i % 512, i);
        if let Some(&v) = lru.get(&(i % 256)) { acc += v; }
    }
    println!("lru_slab_cache ns={} acc={}", t0.elapsed().as_nanos() / 10000, acc);
}
