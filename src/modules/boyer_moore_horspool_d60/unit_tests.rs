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
fn test_unit_boyer_moore_horspool_d60() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::bmh_find_all(b"trusthardtoothbrushes", b"tooth"), vec![9]);
    assert_eq!(crate::core::bmh_find_all(b"aaaaaa", b"aa"), vec![0, 1, 2, 3, 4]);
    assert!(crate::invariants::ensure(true).is_ok());
}
