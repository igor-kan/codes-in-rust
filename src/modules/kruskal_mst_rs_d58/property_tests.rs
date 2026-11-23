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
fn test_prop_kruskal_mst_rs_d58() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let (w, c) = crate::core::kruskal_forest_weight(3, &[(0,1,5),(1,2,3),(0,2,9)]);
    assert_eq!((w, c), (8, 2));
}
