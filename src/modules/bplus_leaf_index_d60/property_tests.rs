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
fn test_prop_bplus_leaf_index_d60() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut idx = crate::core::ChunkedIndex::new(4);
    for i in (0..30i64).rev() { idx.insert(i * 2, i); }
    let scan = idx.range_scan(10, 20);
    assert_eq!(scan.len(), 6);
    for w in scan.windows(2) { assert!(w[0].0 < w[1].0); }
}
