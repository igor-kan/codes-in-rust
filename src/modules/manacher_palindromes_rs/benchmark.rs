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
    let s = b"abacabadabacaba".repeat(64);
    let t0 = Instant::now();
    for _it in 0..200u64 {
        sink = sink.wrapping_add(core::compute_manacher(&s).longest_len as u64);
    }
    let ns = t0.elapsed().as_nanos() as f64 / 200.0;
    println!("manacher_palindromes_rs ns={:.1} sink={}", ns, sink);
}
