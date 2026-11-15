#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::build_sam;
use std::time::Instant;

fn main() {
    let s: Vec<u8> = (0..1024).map(|i| b'a' + ((i * 7 + 3) % 4) as u8).collect();
    let t0 = Instant::now();
    let mut acc = 0u64;
    for _ in 0..100 {
        acc += build_sam(&s).unwrap().distinct_substrings();
    }
    println!("suffix_automaton_rs ns={} acc={}", t0.elapsed().as_nanos() / 100, acc);
}
