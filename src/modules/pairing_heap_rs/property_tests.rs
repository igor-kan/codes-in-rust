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
fn test_parallel_heapsort_parity() {
    let chunks = vec![vec![9, 1, 7, 3, 5], vec![10, 8, 6, 4, 2]];
    let out = crate::concurrent::parallel_sort_slices(&chunks);
    assert_eq!(out[0], vec![1, 3, 5, 7, 9]);
    assert_eq!(out[1], vec![2, 4, 6, 8, 10]);
}
