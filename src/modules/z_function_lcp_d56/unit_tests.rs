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
fn test_unit_z_function_lcp_d56() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let hits = crate::core::z_search(b"abababa", b"aba");
    assert_eq!(hits, vec![0, 2, 4]);
    assert!(crate::invariants::ensure(true).is_ok());
}
