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
    let ac = interface::compile_patterns(&[b"he", b"she", b"hers", b"his"]).unwrap(); let txt = b"ushers_he_she_his_hers".repeat(32);
    let t0 = Instant::now();
    for _it in 0..200u64 {
        sink = sink.wrapping_add(fast_path::count_all_matches(&ac, &txt) as u64);
    }
    let ns = t0.elapsed().as_nanos() as f64 / 200.0;
    println!("aho_corasick_rs ns={:.1} sink={}", ns, sink);
}
