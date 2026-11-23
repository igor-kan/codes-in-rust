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
fn test_prop_lcp_kasai_rs_d58() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let (_, _, dist) = crate::core::suffix_lcp_distinct(b"aaaa");
    assert_eq!(dist, 4);
}
