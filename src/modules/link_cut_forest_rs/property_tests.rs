#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_lct_relink() {
    let mut l = crate::core::LinkCutForest::new(&[5,10,20]);
    l.link(0,1);
    l.link(1,2);
    l.cut(0,1);
    l.link(0,2);
    assert_eq!(l.path_xor(0,1), 27);
   
}
