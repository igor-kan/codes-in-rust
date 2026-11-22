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
fn test_prop_quickselect_kth_d57() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let raw = vec![42, -5, 18, 0, 7, 11];
    let mut sorted = raw.clone();
    sorted.sort();
    for k in 0..raw.len() {
        let mut w = raw.clone();
        assert_eq!(crate::core::quickselect_kth(&mut w, k), sorted[k]);
    }
}
