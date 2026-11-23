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
fn test_prop_aho_corasick_rs_d58() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    assert_eq!(crate::core::ac_match_total(&[b"ab", b"bab"], b"bab"), 2);
}
