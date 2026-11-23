pub fn kruskal_forest_weight(n: usize, edges: &[(usize, usize, i64)]) -> (i64, usize) {
    let mut sorted: Vec<(i64, usize, usize)> = edges.iter().map(|&(u, v, w)| (w, u, v)).collect();
    sorted.sort();
    let mut p: Vec<usize> = (0..n).collect();
    let find = |p: &mut [usize], mut x: usize| -> usize {
        while p[x] != x { p[x] = p[p[x]]; x = p[x]; }
        x
    };
    let mut total = 0i64;
    let mut picked = 0usize;
    for (w, u, v) in sorted {
        let ru = find(&mut p, u);
        let rv = find(&mut p, v);
        if ru != rv {
            p[rv] = ru;
            total += w;
            picked += 1;
        }
    }
    (total, picked)
}
