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
fn test_unit_hopcroft_karp_bip_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let m = crate::core::hopcroft_karp(4, 4, &[(0,0),(0,1),(1,0),(2,2),(2,3),(3,2)]); assert_eq!(m.size, 4);
    assert!(crate::invariants::ensure(true).is_ok());
}
