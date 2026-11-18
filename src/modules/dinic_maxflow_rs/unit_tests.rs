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
fn test_unit_dinic_maxflow_rs() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut g = crate::core::Dinic::new(4); g.add_edge(0,1,10); g.add_edge(0,2,10); g.add_edge(1,2,2); g.add_edge(1,3,8); g.add_edge(2,3,10); assert_eq!(g.max_flow(0,3), 18);
    assert!(crate::invariants::ensure(true).is_ok());
}
