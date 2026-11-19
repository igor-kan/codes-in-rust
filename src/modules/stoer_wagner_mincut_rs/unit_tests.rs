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
fn test_unit_stoer_wagner_mincut_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let g = vec![vec![0,2,0,0,3],vec![2,0,3,0,2],vec![0,3,0,4,0],vec![0,0,4,0,1],vec![3,2,0,1,0]]; assert_eq!(crate::core::stoer_wagner_mincut(&g), 4);
    assert!(crate::invariants::ensure(true).is_ok());
}
