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
fn test_prop_tarjan_bridges_lowlink() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    assert!(crate::core::find_bridges(3, &[(0,1),(1,2),(2,0)]).is_empty());
}
