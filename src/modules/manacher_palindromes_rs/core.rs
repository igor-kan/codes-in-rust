use crate::types::PalindromeProfile;

pub fn compute_manacher(s: &[u8]) -> PalindromeProfile {
    let n = s.len();
    let mut d1 = vec![0usize; n];
    let mut l = 0isize;
    let mut r = -1isize;
    for i in 0..n {
        let mut k = if (i as isize) > r {
            1usize
        } else {
            d1[(l + r - i as isize) as usize].min((r - i as isize + 1) as usize)
        };
        while i >= k && i + k < n && s[i - k] == s[i + k] {
            k += 1;
        }
        d1[i] = k;
        let ri = i as isize + k as isize - 1;
        if ri > r {
            l = i as isize - k as isize + 1;
            r = ri;
        }
    }

    let mut d2 = vec![0usize; n];
    l = 0;
    r = -1;
    for i in 0..n {
        let mut k = if (i as isize) > r {
            0usize
        } else {
            d2[(l + r - i as isize + 1) as usize].min((r - i as isize + 1) as usize)
        };
        while i >= k + 1 && i + k < n && s[i - k - 1] == s[i + k] {
            k += 1;
        }
        d2[i] = k;
        let ri = i as isize + k as isize - 1;
        if ri > r {
            l = i as isize - k as isize;
            r = ri;
        }
    }

    let mut best_start = 0usize;
    let mut best_len = 1usize;
    for i in 0..n {
        let len1 = 2 * d1[i] - 1;
        if len1 > best_len {
            best_len = len1;
            best_start = i + 1 - d1[i];
        }
        let len2 = 2 * d2[i];
        if len2 > best_len {
            best_len = len2;
            best_start = i - d2[i];
        }
    }
    PalindromeProfile { d_odd: d1, d_even: d2, longest_start: best_start, longest_len: best_len }
}
