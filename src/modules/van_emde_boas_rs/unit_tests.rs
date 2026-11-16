#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::build_veb;
use invariants::verify_veb_oracle;
use std::collections::BTreeSet;

#[test]
fn test_veb_insert_succ_delete() {
    let mut v = build_veb(256).unwrap();
    let mut ref_set = BTreeSet::new();
    for &x in &[2usize, 3, 4, 5, 7, 14, 15, 250] {
        v.insert(x);
        ref_set.insert(x);
    }
    assert_eq!(verify_veb_oracle(&v, &ref_set), Ok(()));
    v.delete(2); ref_set.remove(&2);
    v.delete(14); ref_set.remove(&14);
    assert_eq!(verify_veb_oracle(&v, &ref_set), Ok(()));
}
