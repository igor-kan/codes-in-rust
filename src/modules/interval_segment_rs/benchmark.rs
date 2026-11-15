#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::total_covered_length;
use interface::coalesce_intervals;
use std::time::Instant;
use types::Interval;

fn main() {
    let raw: Vec<Interval> = (0..1000).map(|i| Interval { start: i * 3, end: i * 3 + 5 }).collect();
    let t0 = Instant::now();
    let mut sum = 0i64;
    for _ in 0..200 {
        let m = coalesce_intervals(&raw).unwrap();
        sum += total_covered_length(&m);
    }
    println!("interval_segment_rs ns={} sum={}", t0.elapsed().as_nanos() / 200, sum);
}
