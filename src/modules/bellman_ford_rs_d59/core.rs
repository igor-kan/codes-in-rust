pub fn bellman_ford_opt(n: usize, src: usize, edges: &[(usize, usize, i64)]) -> Option<Vec<i64>> {
    let inf = i64::MAX / 4;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    for _ in 0..n.saturating_sub(1) {
        let mut chg = false;
        for &(u, v, w) in edges {
            if dist[u] < inf && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                chg = true;
            }
        }
        if !chg { break; }
    }
    for &(u, v, w) in edges {
        if dist[u] < inf && dist[u] + w < dist[v] { return None; }
    }
    Some(dist)
}
