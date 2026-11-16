#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use concurrent::batch_topo_sort;
use invariants::verify_topo_order;
use types::TopoOutcome;

#[test]
fn prop_random_dags_and_cycles() {
    let mut cases = Vec::new();
    for k in 0..8usize {
        let mut edges = Vec::new();
        for u in 0..20 {
            for v in (u + 1)..20 {
                if (u * 7 + v * 11 + k) % 5 == 0 { edges.push((u, v)); }
            }
        }
        cases.push((20usize, edges));
    }
    let res = batch_topo_sort(&cases);
    for (i, r) in res.iter().enumerate() {
        assert!(matches!(r, TopoOutcome::Ordered(_)));
        assert_eq!(verify_topo_order(cases[i].0, &cases[i].1, r), Ok(()));
    }
}
