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
fn test_unit_convex_hull_andrew_d56() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let pts = vec![(0, 0), (4, 0), (4, 4), (0, 4), (2, 2), (1, 1)];
    let h = crate::core::convex_hull(pts);
    assert_eq!(h, vec![(0, 0), (4, 0), (4, 4), (0, 4)]);
    assert!(crate::invariants::ensure(true).is_ok());
}
