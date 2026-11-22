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
fn test_prop_segment_min_point_d57() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let arr = [10, 5, 14, 2, 11];
    let st = crate::core::IterSegMin::from_slice(&arr);
    for l in 0..arr.len() {
        for r in (l + 1)..=arr.len() {
            assert_eq!(st.query_min(l, r), *arr[l..r].iter().min().unwrap());
        }
    }
}
