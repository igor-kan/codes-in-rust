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
fn test_prop_mo_offline_freq_d56() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let xs = [5, 5, 5, 6];
    assert_eq!(crate::core::distinct_in_ranges(&xs, &[(0, 2), (2, 3)]), vec![1, 2]);
}
