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
fn test_prop_fenwick_prefix_rs_d58() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut fw = crate::core::Fenwick::new(4);
    for i in 0..4 { fw.add(i, (i + 1) as i64); }
    assert_eq!(fw.range_sum(0, 3), 10);
}
