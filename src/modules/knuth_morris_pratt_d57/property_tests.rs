#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test]
fn test_prop_knuth_morris_pratt_d57() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    assert_eq!(crate::core::kmp_non_overlapping(b"rust", b"go"), 0);
}
