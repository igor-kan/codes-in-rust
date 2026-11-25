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
fn test_prop_dijkstra_radix_heap_d60() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let d = crate::core::shortest_distances(5, 0, &[(0,1,10),(1,2,20),(0,2,25),(2,3,5)]);
    assert_eq!(d[3], 30);
    assert_eq!(d[4], u64::MAX);
}
