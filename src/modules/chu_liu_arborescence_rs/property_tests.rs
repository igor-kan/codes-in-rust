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
fn test_prop_chu_liu_arborescence_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    use crate::types::DirEdge; let es = vec![DirEdge{u:0,v:1,w:5},DirEdge{u:1,v:2,w:6}]; assert_eq!(crate::core::edmonds_arborescence(3, 0, es), Some(11));
}
