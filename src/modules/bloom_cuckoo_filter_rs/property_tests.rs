#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use concurrent::parallel_hit_count;
use fast_path::batch_contains;
use interface::make_cuckoo_filter;
use invariants::verify_cuckoo_involution;

#[test]
fn prop_cuckoo_zero_false_negatives_and_low_fpr() {
    let mut cf = make_cuckoo_filter(1024, 99).unwrap();
    let keys: Vec<u64> = (0..600u64).map(|i| i * 31 + 1).collect();
    for &k in &keys {
        cf.insert(k).unwrap();
    }
    assert_eq!(parallel_hit_count(&cf, &keys), 600);
    let probes: Vec<u64> = (100_000..101_000u64).collect();
    let fps = batch_contains(&cf, &probes).into_iter().filter(|&b| b).count();
    assert!(fps < 10);
    assert_eq!(verify_cuckoo_involution(&cf), Ok(()));
}
