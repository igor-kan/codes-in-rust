pub fn bmh_find_all(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    let n = haystack.len();
    let m = needle.len();
    if m == 0 || m > n { return Vec::new(); }
    let mut skip = [m; 256];
    for i in 0..(m - 1) {
        skip[needle[i] as usize] = m - 1 - i;
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i <= n - m {
        let mut j = m;
        while j > 0 && haystack[i + j - 1] == needle[j - 1] {
            j -= 1;
        }
        if j == 0 {
            out.push(i);
            i += 1;
        } else {
            i += skip[haystack[i + m - 1] as usize];
        }
    }
    out
}
