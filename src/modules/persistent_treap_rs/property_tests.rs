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
fn test_prop_persistent_treap_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut pt = crate::core::PersistentTreap::new(); let nd = pt.make(42, 1, None, None); assert_eq!(pt.sum(Some(nd)), 42);
}
