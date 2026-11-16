#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::make_cuckoo_filter;
use invariants::verify_cuckoo_involution;
use validation::load_factor;

#[test]
fn test_cuckoo_insert_contains_delete() {
    let mut cf = make_cuckoo_filter(256, 7).unwrap();
    for k in 0..150u64 {
        cf.insert(k * 17 + 5).unwrap();
    }
    for k in 0..150u64 {
        assert!(cf.contains(k * 17 + 5));
    }
    for k in 0..75u64 {
        assert!(cf.delete(k * 17 + 5));
    }
    for k in 75..150u64 {
        assert!(cf.contains(k * 17 + 5));
    }
    assert!(load_factor(&cf).unwrap() > 0.25);
    assert_eq!(verify_cuckoo_involution(&cf), Ok(()));
}
