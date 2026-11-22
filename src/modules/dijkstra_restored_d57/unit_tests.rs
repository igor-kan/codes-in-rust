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
fn test_unit_dijkstra_restored_d57() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let g = vec![
        vec![(1, 2), (2, 8)],
        vec![(2, 3), (3, 7)],
        vec![(3, 1)],
        vec![],
    ];
    let (c, p) = crate::core::shortest_path_trace(4, &g, 0, 3).unwrap();
    assert_eq!(c, 6);
    assert_eq!(p, vec![0, 1, 2, 3]);
    assert!(crate::invariants::ensure(true).is_ok());
}
