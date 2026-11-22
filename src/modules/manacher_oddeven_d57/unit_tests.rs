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
fn test_unit_manacher_oddeven_d57() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::count_palindromic_substrings(b"aaa"), 6);
    assert_eq!(crate::core::count_palindromic_substrings(b"abc"), 3);
    assert_eq!(crate::core::count_palindromic_substrings(b"abba"), 6);
    assert!(crate::invariants::ensure(true).is_ok());
}
