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
fn test_unit_miller_rabin_u64_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    for p in [2u64, 3, 5, 97, 1_000_000_007, 2_305_843_009_213_693_951] {
        assert!(crate::core::is_prime(p));
    }
    for c in [0u64, 1, 4, 9, 561, 1105, u64::MAX] {
        assert!(!crate::core::is_prime(c));
    }
    let report = crate::core::prime_report(7919);
    assert_eq!(report.candidate, 7919);
    assert!(report.prime);
    assert!(crate::invariants::ensure(true).is_ok());
}
