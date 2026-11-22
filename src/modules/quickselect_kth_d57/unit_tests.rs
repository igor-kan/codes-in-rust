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
fn test_unit_quickselect_kth_d57() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut v = vec![9, 1, 7, 3, 5, 2];
    assert_eq!(crate::core::quickselect_kth(&mut v, 0), 1);
    assert_eq!(crate::core::quickselect_kth(&mut v, 3), 5);
    assert_eq!(crate::core::quickselect_kth(&mut v, 5), 9);
    assert!(crate::invariants::ensure(true).is_ok());
}
