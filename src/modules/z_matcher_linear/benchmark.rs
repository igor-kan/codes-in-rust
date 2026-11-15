#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::count_occurrences_fast;
use std::time::Instant;

fn main() {
    let text = vec![b'a'; 8192];
    let pat = vec![b'a'; 17];
    let t0 = Instant::now();
    let mut hits = 0usize;
    for _ in 0..200 {
        hits += count_occurrences_fast(&text, &pat);
    }
    println!("z_matcher_linear ns={} hits={}", t0.elapsed().as_nanos() / 200, hits);
}
