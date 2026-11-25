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
fn test_unit_dijkstra_radix_heap_d60() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let d = crate::core::shortest_distances(4, 0, &[(0,1,4),(0,2,1),(2,1,2),(1,3,1),(2,3,5)]);
    assert_eq!(d, vec![0, 3, 1, 4]);
    assert!(crate::invariants::ensure(true).is_ok());
}
