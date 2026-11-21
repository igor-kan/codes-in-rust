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
fn test_unit_mo_offline_freq_d56() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let xs = [1, 2, 1, 3, 2, 4, 1];
    let res = crate::core::distinct_in_ranges(&xs, &[(0, 2), (1, 5), (0, 6)]);
    assert_eq!(res, vec![2, 4, 4]);
    assert!(crate::invariants::ensure(true).is_ok());
}
