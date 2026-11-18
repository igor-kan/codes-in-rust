use crate::types::DirEdge;

pub fn edmonds_arborescence(mut n: usize, mut root: usize, mut edges: Vec<DirEdge>) -> Option<i64> {
    let mut res = 0i64;
    let inf = i64::MAX / 4;
    loop {
        let mut in_w = vec![inf; n];
        let mut pre = vec![usize::MAX; n];
        for e in &edges {
            if e.u != e.v && e.w < in_w[e.v] {
                in_w[e.v] = e.w;
                pre[e.v] = e.u;
            }
        }
        for i in 0..n { if i != root && in_w[i] == inf { return None; } }
        let mut tn = 0usize;
        let mut id = vec![usize::MAX; n];
        let mut vis = vec![usize::MAX; n];
        in_w[root] = 0;
        for i in 0..n {
            res += in_w[i];
            let mut v = i;
            while vis[v] != i && id[v] == usize::MAX && v != root {
                vis[v] = i;
                v = pre[v];
            }
            if v != root && id[v] == usize::MAX {
                let mut u = pre[v];
                while u != v { id[u] = tn; u = pre[u]; }
                id[v] = tn;
                tn += 1;
            }
        }
        if tn == 0 { break; }
        for i in 0..n { if id[i] == usize::MAX { id[i] = tn; tn += 1; } }
        for e in &mut edges {
            let v = e.v;
            e.u = id[e.u]; e.v = id[e.v];
            if e.u != e.v { e.w -= in_w[v]; }
        }
        n = tn;
        root = id[root];
    }
    Some(res)
}
