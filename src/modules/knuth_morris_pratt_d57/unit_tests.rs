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
fn test_unit_knuth_morris_pratt_d57() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::kmp_non_overlapping(b"abababa", b"aba"), 2);
    assert_eq!(crate::core::kmp_non_overlapping(b"aaaa", b"aa"), 2);
    assert!(crate::invariants::ensure(true).is_ok());
}
