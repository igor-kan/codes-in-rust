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
fn test_prop_dijkstra_restored_d57() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let g = vec![vec![(1, 5)], vec![]];
    assert!(crate::core::shortest_path_trace(2, &g, 1, 0).is_none());
}
