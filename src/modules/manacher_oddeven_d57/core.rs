pub fn count_palindromic_substrings(s: &[u8]) -> usize {
    let n = s.len();
    if n == 0 { return 0; }
    let mut total = 0usize;
    let mut d1 = vec![0usize; n];
    let (mut l, mut r) = (0isize, -1isize);
    for i in 0..n {
        let mut k = if (i as isize) > r { 1 } else { d1[(l + r - i as isize) as usize].min((r - i as isize + 1) as usize) };
        while i >= k && i + k < n && s[i - k] == s[i + k] { k += 1; }
        d1[i] = k;
        total += k;
        if (i + k) as isize - 1 > r { l = (i + 1 - k) as isize; r = (i + k) as isize - 1; }
    }
    let mut d2 = vec![0usize; n];
    l = 0; r = -1;
    for i in 0..n {
        let mut k = if (i as isize) > r { 0 } else { d2[(l + r - i as isize + 1) as usize].min((r - i as isize + 1) as usize) };
        while i >= k + 1 && i + k < n && s[i - k - 1] == s[i + k] { k += 1; }
        d2[i] = k;
        total += k;
        if (i + k) as isize - 1 > r { l = i as isize - k as isize; r = (i + k) as isize - 1; }
    }
    total
}
