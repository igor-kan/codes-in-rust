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
fn test_unit_lazy_seg_add_rs_d59() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut st = crate::core::LazySegTree::new(6);
    st.add_range(1, 4, 10);
    st.add_range(2, 5, 3);
    assert_eq!(st.query_sum(0, 5), 52);
    assert_eq!(st.query_sum(2, 4), 39);
    assert!(crate::invariants::ensure(true).is_ok());
}
