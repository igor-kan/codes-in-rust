#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::lower_bound_prefix;
use interface::from_slice_linear;
use invariants::verify_all_prefixes;

#[test]
fn test_fenwick_linear_and_lower_bound() {
    let mut bit = from_slice_linear(&[3, 2, 5, 1, 4]);
    assert_eq!(verify_all_prefixes(&bit), Ok(()));
    assert_eq!(bit.range_sum(1, 4), 8); // 2 + 5 + 1
    assert_eq!(lower_bound_prefix(&bit, 6), Some(3)); // 3+2+5=10 >= 6
    bit.add(1, 4); // [3, 6, 5, 1, 4]
    assert_eq!(lower_bound_prefix(&bit, 6), Some(2));
    assert_eq!(verify_all_prefixes(&bit), Ok(()));
}
