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
fn test_prop_min_cost_max_flow_d60() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let (f, c) = crate::core::min_cost_max_flow(3, 0, 2, &[(0,1,4,2),(1,2,4,3),(0,2,2,10)]);
    assert_eq!(f, 6);
    assert_eq!(c, 40);
}
