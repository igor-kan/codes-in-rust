use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::types::DirEdge;

pub fn johnson_apsp(n: usize, edges: &[DirEdge]) -> Vec<Vec<i64>> {
    let inf = i64::MAX / 4;
    let mut h = vec![0i64; n];
    for _ in 0..n {
        let mut chg = false;
        for e in edges {
            if h[e.u] + e.w < h[e.v] { h[e.v] = h[e.u] + e.w; chg = true; }
        }
        if !chg { break; }
    }
    let mut adj = vec![Vec::new(); n];
    for e in edges { adj[e.u].push((e.v, e.w + h[e.u] - h[e.v])); }
    let mut out = vec![vec![inf; n]; n];
    for s in 0..n {
        let mut d = vec![inf; n];
        d[s] = 0;
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0i64, s)));
        while let Some(Reverse((du, u))) = pq.pop() {
            if du > d[u] { continue; }
            for &(v, w) in &adj[u] {
                if du + w < d[v] { d[v] = du + w; pq.push(Reverse((d[v], v))); }
            }
        }
        for v in 0..n { if d[v] < inf { out[s][v] = d[v] - h[s] + h[v]; } }
    }
    out
}
