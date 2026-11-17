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
    for it in 0..500 {
        sink += fast_path::fast_acc(&[1, 2, it as i64]);
    }
    let us = t0.elapsed().as_micros();
    println!("gomory_hu_tree_rs us={us} sink={sink}");
}
