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
fn test_unit_chu_liu_arborescence_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    use crate::types::DirEdge; let es = vec![DirEdge{u:0,v:1,w:10},DirEdge{u:0,v:2,w:20},DirEdge{u:1,v:2,w:2},DirEdge{u:2,v:1,w:3},DirEdge{u:1,v:3,w:15},DirEdge{u:2,v:3,w:4}]; assert_eq!(crate::core::edmonds_arborescence(4, 0, es), Some(16));
    assert!(crate::invariants::ensure(true).is_ok());
}
