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
fn test_unit_hungarian_bipartite_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::hungarian_min_cost(&[vec![1,2,3],vec![2,4,6],vec![3,6,9]]).cost, 10);
    assert!(crate::invariants::ensure(true).is_ok());
}
