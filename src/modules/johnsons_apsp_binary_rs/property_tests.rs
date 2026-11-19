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
fn test_prop_johnsons_apsp_binary_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    use crate::types::DirEdge; let d = crate::core::johnson_apsp(3, &[DirEdge{u:0,v:1,w:10},DirEdge{u:0,v:2,w:2},DirEdge{u:2,v:1,w:-1}]); assert_eq!(d[0][1], 1);
}
