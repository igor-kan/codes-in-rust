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
fn test_prop_zkw_bottom_up_seg_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut st = crate::core::ZkwSeg::new(4); st.set(0, 7); st.set(3, 5); assert_eq!(st.query(0, 3), 12);
}
