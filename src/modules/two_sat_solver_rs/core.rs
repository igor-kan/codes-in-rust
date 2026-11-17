use crate::types::Clause;
pub fn solve_two_sat(n: usize, clauses: &[Clause]) -> Option<Vec<bool>> {
    let nodes = 2 * n;
    let mut adj = vec![Vec::new(); nodes];
    let lit = |v: usize, neg: bool| 2 * v + usize::from(neg);
    for c in clauses {
        let a = lit(c.a, c.na); let b = lit(c.b, c.nb);
        adj[a ^ 1].push(b); adj[b ^ 1].push(a);
    }
    let mut dfn = vec![usize::MAX; nodes]; let mut low = vec![0usize; nodes];
    let mut comp = vec![usize::MAX; nodes]; let mut on_st = vec![false; nodes];
    let mut st = Vec::new(); let mut timer = 0usize; let mut scc = 0usize;
    fn dfs(u: usize, adj: &[Vec<usize>], dfn: &mut [usize], low: &mut [usize], comp: &mut [usize], on_st: &mut [bool], st: &mut Vec<usize>, timer: &mut usize, scc: &mut usize) {
        dfn[u] = *timer; low[u] = *timer; *timer += 1;
        st.push(u); on_st[u] = true;
        for &w in &adj[u] {
            if dfn[w] == usize::MAX { dfs(w, adj, dfn, low, comp, on_st, st, timer, scc); low[u] = low[u].min(low[w]); }
            else if on_st[w] { low[u] = low[u].min(dfn[w]); }
        }
        if low[u] == dfn[u] {
            while let Some(v) = st.pop() {
                on_st[v] = false; comp[v] = *scc;
                if v == u { break; }
            }
            *scc += 1;
        }
    }
    for i in 0..nodes { if dfn[i] == usize::MAX { dfs(i, &adj, &mut dfn, &mut low, &mut comp, &mut on_st, &mut st, &mut timer, &mut scc); } }
    let mut assign = vec![false; n];
    for i in 0..n {
        if comp[2 * i] == comp[2 * i + 1] { return None; }
        assign[i] = comp[2 * i] < comp[2 * i + 1];
    }
    Some(assign)
}
