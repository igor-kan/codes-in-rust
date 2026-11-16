#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::{with_capacity, FlatDict};
use invariants::verify_map_oracle;
use std::collections::HashMap;
use validation::validate_map;

#[test]
fn test_swiss_insert_update_delete() {
    let mut m = with_capacity(16, 42).unwrap();
    let mut ref_m = HashMap::new();
    for k in 0..80u64 {
        m.insert(k * 7, (k as i64) * 10);
        ref_m.insert(k * 7, (k as i64) * 10);
    }
    for k in (0..80u64).step_by(3) {
        assert_eq!(m.remove(k * 7), ref_m.remove(&(k * 7)));
    }
    assert_eq!(validate_map(&m), Ok(()));
    assert_eq!(verify_map_oracle(&m, &ref_m), Ok(()));
}
