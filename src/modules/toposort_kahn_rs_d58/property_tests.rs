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
fn test_prop_toposort_kahn_rs_d58() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    assert!(crate::core::kahn_toposort(2, &[(0, 1), (1, 0)]).is_none());
}
