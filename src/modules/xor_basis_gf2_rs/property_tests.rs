#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_xor_kth_mono() {
    let mut xb = crate::core::XorBasis::default();
    for &v in &[11u64, 29, 43] {
    xb.insert(v);
   
} let ks: Vec<u64> = (0..8).map(|k| xb.kth(k)).collect();
    assert!(ks.windows(2).all(|w| w[0] < w[1]));
   
}
