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
fn test_prop_lazy_seg_add_rs_d59() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut st = crate::core::LazySegTree::new(4);
    st.add_range(0, 3, -5);
    assert_eq!(st.query_sum(1, 2), -10);
}
