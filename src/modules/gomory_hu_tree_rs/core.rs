use std::collections::VecDeque;
use crate::types::GhEdge;
#[derive(Clone)] struct Fe { to: usize, rev: usize, cap: i64 }
fn max_flow(n: usize, edges: &[(usize, usize, i64)], s: usize, t: usize) -> (i64, Vec<bool>) {
    let mut g = vec![Vec::<Fe>::new(); n];
    for &(u, v, c) in edges {
        let a = g[u].len(); let b = g[v].len();
        g[u].push(Fe { to: v, rev: b, cap: c }); g[v].push(Fe { to: u, rev: a, cap: c });
    }
    let mut flow = 0i64;
    let mut lvl = vec![-1i32; n];
    loop {
        lvl.fill(-1); lvl[s] = 0;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for e in &g[u] { if e.cap > 0 && lvl[e.to] == -1 { lvl[e.to] = lvl[u] + 1; q.push_back(e.to); } }
        }
        if lvl[t] == -1 { break; }
        let mut ptr = vec![0usize; n];
        fn dfs(u: usize, t: usize, push: i64, g: &mut [Vec<Fe>], lvl: &[i32], ptr: &mut [usize]) -> i64 {
            if u == t || push == 0 { return push; }
            while ptr[u] < g[u].len() {
                let cid = ptr[u]; ptr[u] += 1;
                let to = g[u][cid].to; let cap = g[u][cid].cap; let rev = g[u][cid].rev;
                if cap > 0 && lvl[to] == lvl[u] + 1 {
                    let tr = dfs(to, t, push.min(cap), g, lvl, ptr);
                    if tr > 0 { g[u][cid].cap -= tr; g[to][rev].cap += tr; return tr; }
                }
            }
            0
        }
        loop {
            let p = dfs(s, t, i64::MAX / 4, &mut g, &lvl, &mut ptr);
            if p == 0 { break; }
            flow += p;
        }
    }
    (flow, lvl.into_iter().map(|x| x != -1).collect())
}
pub fn build_gomory_hu(n: usize, edges: &[(usize, usize, i64)]) -> Vec<GhEdge> {
    let mut p = vec![0usize; n];
    let mut tree = Vec::with_capacity(n.saturating_sub(1));
    for i in 1..n {
        let par = p[i];
        let (cut, side) = max_flow(n, edges, i, par);
        tree.push(GhEdge { u: i, v: par, cap: cut });
        for j in (i + 1)..n { if side[j] && p[j] == par { p[j] = i; } }
    }
    tree
}
