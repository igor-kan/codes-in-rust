use std::collections::VecDeque;
use crate::types::Edge;

pub struct Dinic {
    adj: Vec<Vec<Edge>>,
}

impl Dinic {
    pub fn new(n: usize) -> Self { Self { adj: vec![Vec::new(); n] } }
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        let ru = self.adj[v].len();
        let rv = self.adj[u].len();
        self.adj[u].push(Edge { to: v, rev: ru, cap });
        self.adj[v].push(Edge { to: u, rev: rv, cap: 0 });
    }
    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let n = self.adj.len();
        let mut flow = 0;
        loop {
            let mut level = vec![-1i32; n];
            level[s] = 0;
            let mut q = VecDeque::new();
            q.push_back(s);
            while let Some(u) = q.pop_front() {
                for e in &self.adj[u] {
                    if e.cap > 0 && level[e.to] < 0 {
                        level[e.to] = level[u] + 1;
                        q.push_back(e.to);
                    }
                }
            }
            if level[t] < 0 { break; }
            let mut ptr = vec![0usize; n];
            fn dfs(u: usize, t: usize, pushed: i64, adj: &mut [Vec<Edge>], level: &[i32], ptr: &mut [usize]) -> i64 {
                if pushed == 0 || u == t { return pushed; }
                while ptr[u] < adj[u].len() {
                    let cid = ptr[u];
                    let e = adj[u][cid];
                    if level[e.to] == level[u] + 1 && e.cap > 0 {
                        let tr = dfs(e.to, t, pushed.min(e.cap), adj, level, ptr);
                        if tr > 0 {
                            adj[u][cid].cap -= tr;
                            adj[e.to][e.rev].cap += tr;
                            return tr;
                        }
                    }
                    ptr[u] += 1;
                }
                0
            }
            loop {
                let p = dfs(s, t, i64::MAX / 4, &mut self.adj, &level, &mut ptr);
                if p == 0 { break; }
                flow += p;
            }
        }
        flow
    }
}
