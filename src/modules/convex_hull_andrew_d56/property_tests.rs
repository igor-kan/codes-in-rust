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
fn test_prop_convex_hull_andrew_d56() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let tri = crate::core::convex_hull(vec![(0, 0), (6, 0), (3, 5), (3, 1)]);
    assert_eq!(tri.len(), 3);
}
