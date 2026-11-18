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
fn test_unit_cartesian_tree_rmq_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let ct = crate::core::build_cartesian(&[9, 3, 7, 1, 8, 12, 10, 20, 15, 18, 5]); assert_eq!(crate::core::query_rmq_idx(&ct, 0, 2), Some(1)); assert_eq!(crate::core::query_rmq_idx(&ct, 4, 10), Some(10));
    assert!(crate::invariants::ensure(true).is_ok());
}
