pub fn z_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i <= r {
            z[i] = (r - i + 1).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r + 1 {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

pub fn z_search(text: &[u8], pat: &[u8]) -> Vec<usize> {
    if pat.is_empty() || text.len() < pat.len() {
        return Vec::new();
    }
    let mut joined = Vec::with_capacity(pat.len() + 1 + text.len());
    joined.extend_from_slice(pat);
    joined.push(0);
    joined.extend_from_slice(text);
    let z = z_array(&joined);
    let m = pat.len();
    let mut hits = Vec::new();
    for (idx, &val) in z.iter().enumerate().skip(m + 1) {
        if val >= m {
            hits.push(idx - m - 1);
        }
    }
    hits
}
