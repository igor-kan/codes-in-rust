pub fn held_karp_tsp(dist: &[Vec<i64>]) -> i64 {
    let n = dist.len();
    let inf = 1_000_000_000_000i64;
    let full = (1usize << n) - 1;
    let mut dp = vec![vec![inf; n]; 1usize << n];
    dp[1][0] = 0;
    for mask in 1usize..=full {
        for u in 0..n {
            if (mask & (1usize << u)) == 0 || dp[mask][u] >= inf {
                continue;
            }
            for v in 0..n {
                if (mask & (1usize << v)) != 0 {
                    continue;
                }
                let nmask = mask | (1usize << v);
                dp[nmask][v] = dp[nmask][v].min(dp[mask][u] + dist[u][v]);
            }
        }
    }
    let mut best = inf;
    for u in 1..n {
        best = best.min(dp[full][u] + dist[u][0]);
    }
    if n == 1 { 0 } else { best }
}
