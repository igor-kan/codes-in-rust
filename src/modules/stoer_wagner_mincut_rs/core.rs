pub fn stoer_wagner_mincut(adj: &[Vec<i64>]) -> i64 {
    let n = adj.len();
    let mut w = adj.to_vec();
    let mut vtx: Vec<usize> = (0..n).collect();
    let mut best = i64::MAX / 4;
    for m in (2..=n).rev() {
        let mut wt = vec![0i64; m];
        let mut used = vec![false; m];
        let (mut prev, mut last) = (0usize, 0usize);
        for _ in 0..m {
            let mut sel = None;
            for i in 0..m {
                if !used[i] && sel.map_or(true, |s| wt[i] > wt[s]) { sel = Some(i); }
            }
            let s = sel.unwrap();
            used[s] = true; prev = last; last = s;
            for i in 0..m { if !used[i] { wt[i] += w[vtx[s]][vtx[i]]; } }
        }
        best = best.min(wt[last]);
        let (u, v) = (vtx[prev], vtx[last]);
        for i in 0..m {
            let k = vtx[i];
            if k != u && k != v { w[u][k] += w[v][k]; w[k][u] += w[k][v]; }
        }
        vtx[last] = vtx[m - 1];
    }
    best
}
