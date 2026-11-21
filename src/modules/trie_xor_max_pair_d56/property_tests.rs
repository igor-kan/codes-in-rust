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
fn test_prop_trie_xor_max_pair_d56() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let xs = [4u32, 9, 15, 22, 1];
    let mut brute = 0u32;
    for &a in &xs { for &b in &xs { brute = brute.max(a ^ b); } }
    assert_eq!(crate::core::max_xor_pair(&xs), brute);
}
