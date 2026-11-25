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
fn test_unit_bloom_filter_double_d60() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut bf = crate::core::BloomFilter::new(8, 4);
    for x in [10u64, 20, 30, 99] { bf.insert(x); }
    assert!(bf.may_contain(10));
    assert!(bf.may_contain(99));
    assert!(crate::invariants::ensure(true).is_ok());
}
