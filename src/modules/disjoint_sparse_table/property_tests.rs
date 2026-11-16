#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use concurrent::parallel_query_xor;
use fast_path::batch_range_products;
use interface::build_mod_mul_table;

#[test]
fn prop_dst_non_power2_and_parallel() {
    let arr: Vec<u64> = (0..53u64).map(|i| (i * 29 + 7) % 997).collect();
    let dst = build_mod_mul_table(&arr, 1_000_000_007).unwrap();
    let mut qs = Vec::new();
    for l in 0..arr.len() {
        for r in l..arr.len() {
            qs.push((l, r));
        }
    }
    let seq_xor = batch_range_products(&dst, &qs).into_iter().fold(0u64, |a, x| a ^ x);
    assert_eq!(parallel_query_xor(&dst, &qs), seq_xor);
}
