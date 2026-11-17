#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_lct() {
    let mut l = crate::core::LinkCutForest::new(&[1,2,4,8]);
    assert!(l.link(0,1) && l.link(1,2) && l.link(2,3));
    assert_eq!(l.path_xor(0,3), 15);
    assert!(l.cut(1,2));
    assert_eq!(l.path_xor(0,1), 3);
   
}
