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
fn test_unit_moore_boyer_majority_d56() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut res = crate::core::heavy_hitters(&[3, 2, 3], 2);
    assert_eq!(res, vec![3]);
    res = crate::core::heavy_hitters(&[1, 1, 1, 3, 3, 2, 2, 2], 3);
    assert_eq!(res, vec![1, 2]);
    assert!(crate::invariants::ensure(true).is_ok());
}
