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
fn test_prop_bitmask_tsp_held_karp_d56() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let tri = vec![vec![0, 2, 9], vec![2, 0, 3], vec![9, 3, 0]];
    assert_eq!(crate::core::held_karp_tsp(&tri), 14);
}
