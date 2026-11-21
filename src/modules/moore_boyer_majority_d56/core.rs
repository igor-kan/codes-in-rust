pub fn heavy_hitters(xs: &[i64], k: usize) -> Vec<i64> {
    if k < 2 || xs.is_empty() {
        return Vec::new();
    }
    let slots = k - 1;
    let mut keys: Vec<i64> = Vec::with_capacity(slots);
    let mut cnts: Vec<usize> = Vec::with_capacity(slots);
    for &x in xs {
        if let Some(pos) = keys.iter().position(|&v| v == x) {
            cnts[pos] += 1;
        } else if keys.len() < slots {
            keys.push(x);
            cnts.push(1);
        } else {
            let mut i = 0;
            while i < keys.len() {
                cnts[i] -= 1;
                if cnts[i] == 0 {
                    keys.swap_remove(i);
                    cnts.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    let mut out = Vec::new();
    let thresh = xs.len() / k;
    for &cand in &keys {
        if xs.iter().filter(|&&v| v == cand).count() > thresh {
            out.push(cand);
        }
    }
    out.sort();
    out
}
