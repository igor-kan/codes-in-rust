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
fn test_unit_persistent_treap_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut pt = crate::core::PersistentTreap::new(); let mut r = None; for i in 1..=5 { let nd = pt.make((i*10) as i64, (i as u32).wrapping_mul(2654435761), None, None); r = pt.merge(r, Some(nd)); } assert_eq!(pt.sum(r), 150);
    assert!(crate::invariants::ensure(true).is_ok());
}
