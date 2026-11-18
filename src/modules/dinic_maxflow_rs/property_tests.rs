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
fn test_prop_dinic_maxflow_rs() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut g = crate::core::Dinic::new(3); g.add_edge(0,1,7); g.add_edge(1,2,5); assert_eq!(g.max_flow(0,2), 5);
}
