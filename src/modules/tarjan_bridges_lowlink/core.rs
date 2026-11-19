pub fn find_bridges(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut adj = vec![Vec::new(); n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, i)); adj[v].push((u, i));
    }
    let mut tin = vec![isize::MAX; n];
    let mut low = vec![isize::MAX; n];
    let mut bridges = Vec::new();
    let mut timer = 0isize;
    fn dfs(u: usize, pe: Option<usize>, adj: &[Vec<(usize, usize)>], tin: &mut [isize], low: &mut [isize], timer: &mut isize, bridges: &mut Vec<(usize, usize)>) {
        tin[u] = *timer; low[u] = *timer; *timer += 1;
        for &(v, eid) in &adj[u] {
            if Some(eid) == pe { continue; }
            if tin[v] != isize::MAX {
                low[u] = low[u].min(tin[v]);
            } else {
                dfs(v, Some(eid), adj, tin, low, timer, bridges);
                low[u] = low[u].min(low[v]);
                if low[v] > tin[u] { bridges.push((u.min(v), u.max(v))); }
            }
        }
    }
    for i in 0..n { if tin[i] == isize::MAX { dfs(i, None, &adj, &mut tin, &mut low, &mut timer, &mut bridges); } }
    bridges.sort();
    bridges
}
