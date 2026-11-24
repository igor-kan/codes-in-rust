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
fn test_unit_convex_hull_perim_d59() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::hull_area2(vec![(0,0),(4,0),(4,3),(0,3),(2,1)]), 24);
    assert!(crate::invariants::ensure(true).is_ok());
}
