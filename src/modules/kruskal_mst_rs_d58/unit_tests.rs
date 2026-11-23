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
fn test_unit_kruskal_mst_rs_d58() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let (w, c) = crate::core::kruskal_forest_weight(4, &[(0,1,10),(0,2,6),(0,3,5),(1,3,15),(2,3,4)]);
    assert_eq!((w, c), (19, 3));
    assert!(crate::invariants::ensure(true).is_ok());
}
