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
fn test_prop_bloom_filter_double_d60() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut bf = crate::core::BloomFilter::new(16, 5);
    for k in 1000u64..1080u64 { bf.insert(k); }
    for k in 1000u64..1080u64 { assert!(bf.may_contain(k), "false negative for {k}"); }
}
