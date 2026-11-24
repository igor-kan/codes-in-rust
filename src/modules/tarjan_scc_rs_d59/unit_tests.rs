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
fn test_unit_tarjan_scc_rs_d59() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let (cnt, cid) = crate::core::tarjan_scc(4, &[(0,1),(1,0),(1,2),(2,3),(3,2)]);
    assert_eq!(cnt, 2);
    assert_eq!(cid[0], cid[1]);
    assert_eq!(cid[2], cid[3]);
    assert!(crate::invariants::ensure(true).is_ok());
}
