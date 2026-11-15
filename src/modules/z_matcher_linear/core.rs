pub fn z_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    if n == 0 {
        return z;
    }
    z[0] = n;
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

pub fn find_all_z(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() || text.len() < pattern.len() {
        return Vec::new();
    }
    let plen = pattern.len();
    // Compute Z on pattern, then slide Z-box on text avoiding sentinel byte collisions
    let pz = z_array(pattern);
    let mut offsets = Vec::new();
    let (mut l, mut r) = (0usize, 0usize);
    let n = text.len();
    for i in 0..n {
        let mut k = if i < r { (r - i).min(pz[i - l]) } else { 0 };
        while k < plen && i + k < n && pattern[k] == text[i + k] {
            k += 1;
        }
        if k == plen {
            offsets.push(i);
        }
        if i + k > r {
            l = i;
            r = i + k;
        }
    }
    offsets
}
