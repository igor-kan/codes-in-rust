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
    let mut v = interface::build_veb(4096).unwrap(); for k in (0..4096).step_by(7) { v.insert(k); }
    let t0 = Instant::now();
    for _it in 0..200u64 {
        for q in 0..256usize { sink = sink.wrapping_add(v.successor((q * 13 + _it as usize) & 4095).unwrap_or(0) as u64); }
    }
    let ns = t0.elapsed().as_nanos() as f64 / 200.0;
    println!("van_emde_boas_rs ns={:.1} sink={}", ns, sink);
}
