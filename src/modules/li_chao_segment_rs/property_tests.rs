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
fn test_prop_li_chao_segment_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut lc = crate::core::LiChao::new(32); lc.add_line(3, 1); lc.add_line(1, 10); assert_eq!(lc.query(2), 7);
}
