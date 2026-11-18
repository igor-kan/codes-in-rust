use crate::types::ObstResult;

pub fn solve_obst(freq: &[i64]) -> ObstResult {
    let n = freq.len();
    let mut pref = vec![0i64; n + 1];
    for i in 0..n { pref[i + 1] = pref[i] + freq[i]; }
    let mut dp = vec![vec![0i64; n]; n];
    let mut opt = vec![vec![0usize; n]; n];
    for i in 0..n { dp[i][i] = freq[i]; opt[i][i] = i; }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            let w = pref[j + 1] - pref[i];
            let mut best = i64::MAX / 4;
            let mut br = i;
            for r in opt[i][j - 1]..=opt[i + 1][j] {
                let left = if r > i { dp[i][r - 1] } else { 0 };
                let right = if r < j { dp[r + 1][j] } else { 0 };
                let cand = left + right + w;
                if cand < best { best = cand; br = r; }
            }
            dp[i][j] = best; opt[i][j] = br;
        }
    }
    ObstResult { cost: dp[0][n - 1], root: opt[0][n - 1] }
}
