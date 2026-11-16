#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use concurrent::parallel_read_checksum;
use fast_path::batch_get;
use interface::{with_capacity, FlatDict};
use invariants::verify_map_oracle;
use std::collections::HashMap;

#[test]
fn prop_swiss_oracle_and_parallel_reads() {
    let mut m = with_capacity(8, 1337).unwrap();
    let mut oracle = HashMap::new();
    let mut rng = 0x123456789ABCDEFu64;
    for step in 0..400u64 {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let k = (rng >> 32) % 120;
        if step % 4 == 0 {
            assert_eq!(m.remove(k), oracle.remove(&k));
        } else {
            let v = (step as i64) * 3 - 500;
            assert_eq!(m.insert(k, v), oracle.insert(k, v));
        }
    }
    assert_eq!(verify_map_oracle(&m, &oracle), Ok(()));
    let keys: Vec<u64> = oracle.keys().copied().collect();
    let single: i64 = batch_get(&m, &keys).into_iter().flatten().sum();
    assert_eq!(parallel_read_checksum(&m, &keys), single);
}
