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
fn test_unit_segment_min_point_d57() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut st = crate::core::IterSegMin::from_slice(&[8, 3, 9, 1, 7, 4]);
    assert_eq!(st.query_min(0, 6), 1);
    st.update(3, 12);
    assert_eq!(st.query_min(0, 6), 3);
    assert!(crate::invariants::ensure(true).is_ok());
}
