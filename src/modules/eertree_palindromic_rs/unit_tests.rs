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
fn test_unit_eertree_palindromic_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let et = crate::core::Eertree::new("ertre"); assert_eq!(et.distinct_count(), 5);
    assert!(crate::invariants::ensure(true).is_ok());
}
