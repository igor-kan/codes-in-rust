pub fn kmp_non_overlapping(text: &[u8], pat: &[u8]) -> usize {
    if pat.is_empty() || text.len() < pat.len() { return 0; }
    let m = pat.len();
    let mut pi = vec![0usize; m];
    for i in 1..m {
        let mut j = pi[i - 1];
        while j > 0 && pat[i] != pat[j] { j = pi[j - 1]; }
        if pat[i] == pat[j] { j += 1; }
        pi[i] = j;
    }
    let mut j = 0usize;
    let mut hits = 0usize;
    for &b in text {
        while j > 0 && b != pat[j] { j = pi[j - 1]; }
        if b == pat[j] { j += 1; }
        if j == m {
            hits += 1;
            j = 0;
        }
    }
    hits
}
