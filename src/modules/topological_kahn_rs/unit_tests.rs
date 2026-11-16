#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::is_dag_linear;
use interface::lex_topo_sort;
use invariants::verify_topo_order;
use types::TopoOutcome;

#[test]
fn test_kahn_lex_order_and_cycle() {
    let edges = [(5usize, 2), (5, 0), (4, 0), (4, 1), (2, 3), (3, 1)];
    let res = lex_topo_sort(6, &edges).unwrap();
    assert_eq!(res, TopoOutcome::Ordered(vec![4, 5, 0, 2, 3, 1]));
    assert_eq!(verify_topo_order(6, &edges, &res), Ok(()));
    assert!(!is_dag_linear(3, &[(0, 1), (1, 2), (2, 0)]));
}
