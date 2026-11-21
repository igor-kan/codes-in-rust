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
fn test_unit_trie_xor_max_pair_d56() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::max_xor_pair(&[3, 10, 5, 25, 2, 8]), 28);
    assert_eq!(crate::core::max_xor_pair(&[14, 70, 53, 83, 49, 91]), 119);
    assert!(crate::invariants::ensure(true).is_ok());
}
