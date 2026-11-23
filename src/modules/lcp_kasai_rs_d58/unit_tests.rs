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
fn test_unit_lcp_kasai_rs_d58() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let (_, lcp, distinct) = crate::core::suffix_lcp_distinct(b"banana");
    assert_eq!(*lcp.iter().max().unwrap(), 3);
    assert_eq!(distinct, 15);
    assert!(crate::invariants::ensure(true).is_ok());
}
