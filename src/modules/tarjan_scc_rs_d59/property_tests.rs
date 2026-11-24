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
fn test_prop_tarjan_scc_rs_d59() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let (cnt, _) = crate::core::tarjan_scc(3, &[(0,1),(1,2)]);
    assert_eq!(cnt, 3);
}
