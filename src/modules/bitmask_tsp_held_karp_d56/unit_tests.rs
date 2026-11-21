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
fn test_unit_bitmask_tsp_held_karp_d56() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let d = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    assert_eq!(crate::core::held_karp_tsp(&d), 80);
    assert!(crate::invariants::ensure(true).is_ok());
}
