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
fn test_unit_edit_lev_rs_d59() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::lev_distance_bytes(b"kitten", b"sitting"), 3);
    assert_eq!(crate::core::lev_distance_bytes(b"rust", b"rust"), 0);
    assert!(crate::invariants::ensure(true).is_ok());
}
