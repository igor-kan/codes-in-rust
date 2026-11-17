#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

fn main() {
    let t0 = std::time::Instant::now();
    let mut sink: i64 = 0;
    let data: Vec<i64> = (0..128).map(|i| (i * 43) % 128).collect(); for it in 0..200 { let s = fast_path::fast_heapsort(&data); sink += s[it % 128]; }
    let us = t0.elapsed().as_micros();
    println!("pairing_heap_rs us={us} sink={sink}");
}
