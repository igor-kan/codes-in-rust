#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::count_matches;
use interface::compile_horspool;
use std::time::Instant;

fn main() {
    let mut hay = vec![b'x'; 16384];
    for i in (0..16384).step_by(97) {
        if i + 6 <= hay.len() { hay[i..i + 6].copy_from_slice(b"needle"); }
    }
    let tbl = compile_horspool(b"needle").unwrap();
    let t0 = Instant::now();
    let mut hits = 0usize;
    for _ in 0..500 {
        hits += count_matches(&tbl, &hay);
    }
    println!("boyer_moore_horspool ns={} hits={}", t0.elapsed().as_nanos() / 500, hits);
}
