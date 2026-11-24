pub fn primitive_period(s: &[u8]) -> usize {
    let n = s.len();
    if n == 0 { return 0; }
    let mut pi = vec![0usize; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] { j = pi[j - 1]; }
        if s[i] == s[j] { j += 1; }
        pi[i] = j;
    }
    let cand = n - pi[n - 1];
    if n % cand == 0 { cand } else { n }
}
