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
fn test_prop_edit_lev_rs_d59() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    assert_eq!(crate::core::lev_distance_bytes(b"sunday", b"saturday"), crate::core::lev_distance_bytes(b"saturday", b"sunday"));
}
