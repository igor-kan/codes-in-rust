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
fn test_prop_stoer_wagner_mincut_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let g = vec![vec![0,10,0],vec![10,0,2],vec![0,2,0]]; assert_eq!(crate::core::stoer_wagner_mincut(&g), 2);
}
