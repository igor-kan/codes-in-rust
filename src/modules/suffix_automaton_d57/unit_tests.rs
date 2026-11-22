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
fn test_unit_suffix_automaton_d57() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::sam_distinct_substrings(b"abcbc"), 12);
    assert_eq!(crate::core::sam_distinct_substrings(b"aaaa"), 4);
    assert_eq!(crate::core::sam_distinct_substrings(b"abcd"), 10);
    assert!(crate::invariants::ensure(true).is_ok());
}
