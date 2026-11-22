use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn shortest_path_trace(n: usize, adj: &[Vec<(usize, i64)>], src: usize, dst: usize) -> Option<(i64, Vec<usize>)> {
    let mut dist = vec![i64::MAX; n];
    let mut parent = vec![usize::MAX; n];
    let mut pq = BinaryHeap::new();
    dist[src] = 0;
    pq.push(Reverse((0i64, src)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] { continue; }
        if u == dst { break; }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                parent[v] = u;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    if dist[dst] == i64::MAX { return None; }
    let mut path = Vec::new();
    let mut cur = dst;
    while cur != usize::MAX {
        path.push(cur);
        if cur == src { break; }
        cur = parent[cur];
    }
    path.reverse();
    Some((dist[dst], path))
}
