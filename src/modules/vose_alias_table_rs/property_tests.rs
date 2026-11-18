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
fn test_prop_vose_alias_table_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let t = crate::core::build_alias(&[2.0, 2.0]); assert_eq!(crate::core::sample_alias(&t, 0, 0.2), 0);
}
