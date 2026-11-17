use std::collections::VecDeque;
use crate::types::MatchOut;
pub fn blossom_match(n: usize, edges: &[(usize, usize)]) -> MatchOut {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges { adj[u].push(v); adj[v].push(u); }
    let mut mt: Vec<Option<usize>> = vec![None; n];
    let mut p: Vec<Option<usize>> = vec![None; n];
    let mut base: Vec<usize> = (0..n).collect();
    let mut used = vec![false; n];
    let mut blos = vec![false; n];
    let lca = |mut a: usize, mut b: usize, base: &[usize], mt: &[Option<usize>], p: &[Option<usize>]| -> usize {
        let mut vis = vec![false; n];
        loop { a = base[a]; vis[a] = true; if let Some(m) = mt[a] { a = p[m].unwrap(); } else { break; } }
        loop { b = base[b]; if vis[b] { return b; } b = p[mt[b].unwrap()].unwrap(); }
    };
    let mut pairs = 0usize;
    for root in 0..n {
        if mt[root].is_some() { continue; }
        used.fill(false); p.fill(None);
        for i in 0..n { base[i] = i; }
        let mut q = VecDeque::new();
        used[root] = true; q.push_back(root);
        let mut found = None;
        'bfs: while let Some(v) = q.pop_front() {
            for &to in &adj[v] {
                if base[v] == base[to] || mt[v] == Some(to) { continue; }
                if to == root || (mt[to].is_some() && p[mt[to].unwrap()].is_some()) {
                    let cb = lca(v, to, &base, &mt, &p);
                    blos.fill(false);
                    for &(mut x, mut ch) in &[(v, to), (to, v)] {
                        while base[x] != cb {
                            blos[base[x]] = true; blos[base[mt[x].unwrap()]] = true;
                            p[x] = Some(ch); ch = mt[x].unwrap(); x = p[ch].unwrap();
                        }
                    }
                    for i in 0..n {
                        if blos[base[i]] { base[i] = cb; if !used[i] { used[i] = true; q.push_back(i); } }
                    }
                } else if p[to].is_none() {
                    p[to] = Some(v);
                    if let Some(m) = mt[to] { used[m] = true; q.push_back(m); }
                    else { found = Some(to); break 'bfs; }
                }
            }
        }
        if let Some(mut v) = found {
            pairs += 1;
            loop {
                let pv = p[v].unwrap(); let ppv = mt[pv];
                mt[v] = Some(pv); mt[pv] = Some(v);
                match ppv { Some(nx) => v = nx, None => break }
            }
        }
    }
    MatchOut { pairs, partner: mt }
}
