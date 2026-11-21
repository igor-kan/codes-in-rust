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
fn test_prop_z_function_lcp_d56() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let t = b"rustacean_rust_trust";
    let p = b"rust";
    for pos in crate::core::z_search(t, p) {
        assert_eq!(&t[pos..pos + p.len()], p);
    }
}
