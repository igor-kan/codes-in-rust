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
fn test_prop_moore_boyer_majority_d56() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let xs = vec![7, 7, 1, 7, 2, 7, 3];
    let hh = crate::core::heavy_hitters(&xs, 2);
    assert_eq!(hh, vec![7]);
}
