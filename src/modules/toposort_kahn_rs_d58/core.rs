use std::collections::VecDeque;

pub fn kahn_toposort(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    let mut ind = vec![0usize; n];
    for &(u, v) in edges {
        adj[u].push(v);
        ind[v] += 1;
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if ind[i] == 0 { q.push_back(i); }
    }
    let mut out = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        out.push(u);
        for &v in &adj[u] {
            ind[v] -= 1;
            if ind[v] == 0 { q.push_back(v); }
        }
    }
    if out.len() == n { Some(out) } else { None }
}
