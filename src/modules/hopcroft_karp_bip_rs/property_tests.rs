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
fn test_prop_hopcroft_karp_bip_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let m = crate::core::hopcroft_karp(3, 2, &[(0,0),(1,0),(2,1)]); assert_eq!(m.size, 2);
}
