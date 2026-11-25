use std::collections::VecDeque;
use crate::types::McEdge;

pub fn min_cost_max_flow(n: usize, s: usize, t: usize, raw: &[(usize, usize, i64, i64)]) -> (i64, i64) {
    let mut g = vec![Vec::<McEdge>::new(); n];
    for &(u, v, cap, cost) in raw {
        let a = McEdge { to: v, rev: g[v].len(), cap, cost };
        let b = McEdge { to: u, rev: g[u].len(), cap: 0, cost: -cost };
        g[u].push(a);
        g[v].push(b);
    }
    let mut flow = 0i64;
    let mut cost = 0i64;
    let inf = i64::MAX / 4;
    loop {
        let mut dist = vec![inf; n];
        let mut pv = vec![usize::MAX; n];
        let mut pe = vec![usize::MAX; n];
        let mut inq = vec![false; n];
        let mut q = VecDeque::new();
        dist[s] = 0;
        q.push_back(s);
        inq[s] = true;
        while let Some(u) = q.pop_front() {
            inq[u] = false;
            for (ei, e) in g[u].iter().enumerate() {
                if e.cap > 0 && dist[u] + e.cost < dist[e.to] {
                    dist[e.to] = dist[u] + e.cost;
                    pv[e.to] = u;
                    pe[e.to] = ei;
                    if !inq[e.to] {
                        q.push_back(e.to);
                        inq[e.to] = true;
                    }
                }
            }
        }
        if dist[t] == inf { break; }
        let mut push = i64::MAX;
        let mut v = t;
        while v != s {
            let u = pv[v];
            let ei = pe[v];
            push = push.min(g[u][ei].cap);
            v = u;
        }
        v = t;
        while v != s {
            let u = pv[v];
            let ei = pe[v];
            let rev = g[u][ei].rev;
            g[u][ei].cap -= push;
            g[v][rev].cap += push;
            v = u;
        }
        flow += push;
        cost += push * dist[t];
    }
    (flow, cost)
}
