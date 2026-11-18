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
fn test_prop_cartesian_tree_rmq_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let ct = crate::core::build_cartesian(&[5, 2, 8]); assert_eq!(crate::core::query_rmq_idx(&ct, 0, 2), Some(1));
}
