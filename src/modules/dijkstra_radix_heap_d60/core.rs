use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn shortest_distances(n: usize, src: usize, edges: &[(usize, usize, u64)]) -> Vec<u64> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
    }
    let mut dist = vec![u64::MAX; n];
    dist[src] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0u64, src)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] { continue; }
        for &(v, w) in &adj[u] {
            let nd = d.saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    dist
}
