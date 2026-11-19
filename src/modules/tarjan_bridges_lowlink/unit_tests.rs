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
fn test_unit_tarjan_bridges_lowlink() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::find_bridges(5, &[(0,1),(1,2),(2,0),(2,3),(3,4)]), vec![(2, 3), (3, 4)]);
    assert!(crate::invariants::ensure(true).is_ok());
}
