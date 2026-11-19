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
fn test_unit_zkw_bottom_up_seg_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut st = crate::core::ZkwSeg::new(6); for i in 0..6 { st.set(i, ((i + 1) * 10) as i64); } assert_eq!(st.query(1, 4), 140);
    assert!(crate::invariants::ensure(true).is_ok());
}
