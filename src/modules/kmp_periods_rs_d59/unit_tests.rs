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
fn test_unit_kmp_periods_rs_d59() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::primitive_period(b"ababab"), 2);
    assert_eq!(crate::core::primitive_period(b"abcab"), 5);
    assert!(crate::invariants::ensure(true).is_ok());
}
