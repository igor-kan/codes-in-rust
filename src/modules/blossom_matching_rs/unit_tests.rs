#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_blossom_pentagon() {
    let e = [(0,1),(1,2),(2,3),(3,4),(4,0),(1,5)];
    let r = crate::core::blossom_match(6, &e);
    assert_eq!(r.pairs, 3);
   
}
