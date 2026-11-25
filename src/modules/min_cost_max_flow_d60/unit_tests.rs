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
fn test_unit_min_cost_max_flow_d60() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let (f, c) = crate::core::min_cost_max_flow(4, 0, 3, &[(0,1,2,1),(0,2,2,5),(1,3,1,1),(1,2,1,1),(2,3,2,1)]);
    assert_eq!(f, 3);
    assert_eq!(c, 11);
    assert!(crate::invariants::ensure(true).is_ok());
}
