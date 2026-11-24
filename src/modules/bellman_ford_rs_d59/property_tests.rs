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
fn test_prop_bellman_ford_rs_d59() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    assert!(crate::core::bellman_ford_opt(2, 0, &[(0,1,1),(1,0,-2)]).is_none());
}
