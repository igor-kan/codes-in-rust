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
fn test_unit_toposort_kahn_rs_d58() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let ord = crate::core::kahn_toposort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]).unwrap();
    assert_eq!(ord.len(), 4);
    assert_eq!(ord[0], 0);
    assert_eq!(ord[3], 3);
    assert!(crate::invariants::ensure(true).is_ok());
}
