use crate::types::TopoOutcome;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn kahn_min_heap(n: usize, edges: &[(usize, usize)]) -> TopoOutcome {
    let mut adj = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];
    for &(u, v) in edges {
        adj[u].push(v);
        indeg[v] += 1;
    }
    let mut heap = BinaryHeap::new();
    for v in 0..n {
        if indeg[v] == 0 {
            heap.push(Reverse(v));
        }
    }
    let mut order = Vec::with_capacity(n);
    while let Some(Reverse(u)) = heap.pop() {
        order.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                heap.push(Reverse(v));
            }
        }
    }
    if order.len() == n {
        TopoOutcome::Ordered(order)
    } else {
        let blocked: Vec<usize> = (0..n).filter(|&v| indeg[v] > 0).collect();
        TopoOutcome::HasCycle(blocked)
    }
}
