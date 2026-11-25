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
fn test_unit_bplus_leaf_index_d60() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut idx = crate::core::ChunkedIndex::new(3);
    for k in [10, 4, 7, 1, 15, 12, 3, 9] { idx.insert(k, k * 10); }
    assert_eq!(idx.range_scan(4, 12), vec![(4,40),(7,70),(9,90),(10,100),(12,120)]);
    assert!(crate::invariants::ensure(true).is_ok());
}
