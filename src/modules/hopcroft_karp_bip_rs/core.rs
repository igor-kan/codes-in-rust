use std::collections::VecDeque;
use crate::types::BipMatch;

pub fn hopcroft_karp(n_l: usize, n_r: usize, edges: &[(usize, usize)]) -> BipMatch {
    let mut adj = vec![Vec::new(); n_l];
    for &(u, v) in edges { adj[u].push(v); }
    let mut pair_u = vec![None; n_l];
    let mut pair_v = vec![None; n_r];
    let mut dist = vec![0usize; n_l];
    fn bfs(n_l: usize, adj: &[Vec<usize>], pair_u: &[Option<usize>], pair_v: &[Option<usize>], dist: &mut [usize]) -> bool {
        let mut q = VecDeque::new();
        for u in 0..n_l {
            if pair_u[u].is_none() { dist[u] = 0; q.push_back(u); }
            else { dist[u] = usize::MAX; }
        }
        let mut found = false;
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                match pair_v[v] {
                    None => found = true,
                    Some(u2) if dist[u2] == usize::MAX => { dist[u2] = dist[u] + 1; q.push_back(u2); }
                    _ => {}
                }
            }
        }
        found
    }
    fn dfs(u: usize, adj: &[Vec<usize>], pair_u: &mut [Option<usize>], pair_v: &mut [Option<usize>], dist: &mut [usize]) -> bool {
        for &v in &adj[u] {
            let ok = match pair_v[v] {
                None => true,
                Some(u2) => dist[u2] == dist[u] + 1 && dfs(u2, adj, pair_u, pair_v, dist),
            };
            if ok { pair_u[u] = Some(v); pair_v[v] = Some(u); return true; }
        }
        dist[u] = usize::MAX;
        false
    }
    let mut size = 0;
    while bfs(n_l, &adj, &pair_u, &pair_v, &mut dist) {
        for u in 0..n_l {
            if pair_u[u].is_none() && dfs(u, &adj, &mut pair_u, &mut pair_v, &mut dist) { size += 1; }
        }
    }
    BipMatch { size, pair_u }
}
