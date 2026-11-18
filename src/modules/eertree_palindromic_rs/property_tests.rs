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
fn test_prop_eertree_palindromic_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let et = crate::core::Eertree::new("aaaaa"); assert_eq!(et.distinct_count(), 5);
}
