#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_gh_k4() {
    let mut e=Vec::new();
    for i in 0..4 {
    for j in i+1..4 {
    e.push((i,j,5));
   
}
} let t = crate::core::build_gomory_hu(4, &e);
    assert!(t.iter().all(|x| x.cap == 15));
   
}
