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
fn test_unit_bellman_ford_rs_d59() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let d = crate::core::bellman_ford_opt(4, 0, &[(0,1,5),(0,2,4),(2,1,-2),(1,3,-3)]).unwrap();
    assert_eq!(d[1], 2);
    assert_eq!(d[3], -1);
    assert!(crate::invariants::ensure(true).is_ok());
}
