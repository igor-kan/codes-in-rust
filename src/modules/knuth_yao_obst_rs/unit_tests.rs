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
fn test_unit_knuth_yao_obst_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::solve_obst(&[34, 8, 50]), crate::types::ObstResult { cost: 142, root: 2 });
    assert!(crate::invariants::ensure(true).is_ok());
}
