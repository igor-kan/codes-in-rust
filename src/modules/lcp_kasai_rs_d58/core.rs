pub fn suffix_lcp_distinct(s: &[u8]) -> (Vec<usize>, Vec<usize>, usize) {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&a, &b| s[a..].cmp(&s[b..]));
    let mut rk = vec![0usize; n];
    for i in 0..n { rk[sa[i]] = i; }
    let mut lcp = vec![0usize; n];
    let mut k = 0usize;
    for i in 0..n {
        if rk[i] + 1 == n { k = 0; continue; }
        let j = sa[rk[i] + 1];
        while i + k < n && j + k < n && s[i + k] == s[j + k] { k += 1; }
        lcp[rk[i]] = k;
        if k > 0 { k -= 1; }
    }
    let lcp_sum: usize = lcp.iter().sum();
    let distinct = n * (n + 1) / 2 - lcp_sum;
    (sa, lcp, distinct)
}
