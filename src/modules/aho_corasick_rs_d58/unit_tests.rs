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
fn test_unit_aho_corasick_rs_d58() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::ac_match_total(&[b"he", b"she", b"hers", b"his"], b"ahishers"), 4);
    assert!(crate::invariants::ensure(true).is_ok());
}
