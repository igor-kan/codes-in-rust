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
fn test_unit_boruvka_mst_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let edges = [
        crate::types::WeightedEdge { u: 0, v: 1, w: 4 },
        crate::types::WeightedEdge { u: 0, v: 2, w: 3 },
        crate::types::WeightedEdge { u: 1, v: 2, w: 1 },
        crate::types::WeightedEdge { u: 1, v: 3, w: 2 },
        crate::types::WeightedEdge { u: 2, v: 3, w: 5 },
    ];
    let summary = crate::core::boruvka_mst(4, &edges);
    assert_eq!(summary.weight, 6);
    assert_eq!(summary.edges, 3);
    let forest = [
        crate::types::WeightedEdge { u: 0, v: 1, w: 3 },
        crate::types::WeightedEdge { u: 2, v: 3, w: 4 },
    ];
    let sparse = crate::core::boruvka_mst(4, &forest);
    assert_eq!(sparse.weight, 7);
    assert_eq!(sparse.edges, 2);
    let single = crate::core::boruvka_mst(1, &[]);
    assert_eq!(single.weight, 0);
    assert_eq!(single.edges, 0);
    assert!(crate::invariants::ensure(true).is_ok());
}
