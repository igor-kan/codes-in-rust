#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_gh_chain() {
    let t = crate::core::build_gomory_hu(4, &[(0,1,10),(1,2,4),(2,3,7)]);
    assert_eq!(t.iter().map(|e| e.cap).min(), Some(4));
   
}
