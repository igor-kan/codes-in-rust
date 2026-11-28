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
fn test_prop_boruvka_mst_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    fn reference_msf(n: usize, edges: &[crate::types::WeightedEdge]) -> (i64, usize) {
        let mut sorted = edges.to_vec();
        sorted.sort_by_key(|e| e.w);
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        let mut weight = 0i64;
        let mut used = 0usize;
        for e in sorted {
            let ra = find(&mut parent, e.u);
            let rb = find(&mut parent, e.v);
            if ra != rb {
                parent[ra] = rb;
                weight += e.w;
                used += 1;
            }
        }
        (weight, used)
    }
    let mut seed = 0x2545f4914f6cdd1du64;
    let mut next = || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        seed
    };
    for trial in 0..40usize {
        let n = 2 + trial % 7;
        let mut edges = Vec::new();
        for _ in 0..3 * n {
            let u = (next() % n as u64) as usize;
            let v = (next() % n as u64) as usize;
            let w = (next() % 50) as i64 - 10;
            edges.push(crate::types::WeightedEdge { u, v, w });
        }
        let got = crate::core::boruvka_mst(n, &edges);
        let (want_weight, want_edges) = reference_msf(n, &edges);
        assert_eq!(got.weight, want_weight, "weight mismatch in trial {trial}");
        assert_eq!(got.edges, want_edges, "edge count mismatch in trial {trial}");
        assert!(got.edges < n);
    }
}
