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
fn test_unit_johnsons_apsp_binary_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    use crate::types::DirEdge; let d = crate::core::johnson_apsp(4, &[DirEdge{u:0,v:1,w:4},DirEdge{u:0,v:2,w:5},DirEdge{u:1,v:2,w:-3},DirEdge{u:2,v:3,w:2}]); assert_eq!(d[0][3], 3); assert_eq!(d[1][3], -1);
    assert!(crate::invariants::ensure(true).is_ok());
}
