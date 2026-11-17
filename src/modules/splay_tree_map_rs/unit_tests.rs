#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_splay() {
    let mut m = crate::core::SplayMap::new();
    for i in (0..16).rev() {
    m.insert(i*2, i+100);
   
} assert_eq!(m.get(14), Some(107));
    assert_eq!(m.get(15), None);
   
}
