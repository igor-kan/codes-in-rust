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
fn test_unit_li_chao_segment_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut lc = crate::core::LiChao::new(64); lc.add_line(2, 10); lc.add_line(-1, 50); lc.add_line(0, 25); assert_eq!(lc.query(5), 20); assert_eq!(lc.query(40), 10);
    assert!(crate::invariants::ensure(true).is_ok());
}
