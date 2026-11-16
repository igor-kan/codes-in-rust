#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::build_mod_mul_table;
use invariants::verify_dst_oracle;
use validation::validate_range;

#[test]
fn test_dst_all_subranges() {
    let arr: Vec<u64> = (1..=37).map(|x| x * 13 + 5).collect();
    let dst = build_mod_mul_table(&arr, 1_000_000_007).unwrap();
    assert_eq!(validate_range(&dst, 0, arr.len() - 1), Ok(()));
    assert_eq!(verify_dst_oracle(&dst, &arr), Ok(()));
}
