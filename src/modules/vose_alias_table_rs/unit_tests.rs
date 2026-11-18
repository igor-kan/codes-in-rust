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
fn test_unit_vose_alias_table_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let t = crate::core::build_alias(&[1.0, 3.0]); assert_eq!(crate::core::sample_alias(&t, 1, 0.9), 1);
    assert!(crate::invariants::ensure(true).is_ok());
}
