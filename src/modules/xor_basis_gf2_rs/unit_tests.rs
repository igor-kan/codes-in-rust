#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_xor_basis() {
    let mut xb = crate::core::XorBasis::default();
    xb.insert(3);
    xb.insert(5);
    assert!(!xb.insert(6));
    assert_eq!(xb.max_xor(0), 6);
    assert_eq!(xb.kth(2), 5);
   
}
