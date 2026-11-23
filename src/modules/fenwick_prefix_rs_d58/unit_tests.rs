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
fn test_unit_fenwick_prefix_rs_d58() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut fw = crate::core::Fenwick::new(5);
    fw.add(1, 5); fw.add(3, 10); fw.add(4, -3);
    assert_eq!(fw.range_sum(1, 4), 12);
    assert!(crate::invariants::ensure(true).is_ok());
}
