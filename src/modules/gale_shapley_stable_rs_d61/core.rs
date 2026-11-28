use crate::types::StablePair;

pub fn stable_match(prefs_a: &[Vec<usize>], prefs_b: &[Vec<usize>]) -> Vec<usize> {
    let n = prefs_a.len();
    if n == 0 {
        return Vec::new();
    }
    let mut rank_b = vec![vec![0usize; n]; n];
    for (b, list) in prefs_b.iter().enumerate().take(n) {
        for (r, &a) in list.iter().enumerate() {
            if a < n {
                rank_b[b][a] = r;
            }
        }
    }
    let mut next = vec![0usize; n];
    let mut match_a = vec![usize::MAX; n];
    let mut match_b = vec![usize::MAX; n];
    let mut queue: Vec<usize> = (0..n).collect();
    let mut head = 0usize;
    while head < queue.len() {
        let a = queue[head];
        head += 1;
        if match_a[a] != usize::MAX {
            continue;
        }
        while next[a] < prefs_a[a].len() {
            let b = prefs_a[a][next[a]];
            next[a] += 1;
            if b >= n {
                continue;
            }
            if match_b[b] == usize::MAX {
                match_a[a] = b;
                match_b[b] = a;
                break;
            }
            let cur = match_b[b];
            if rank_b[b][a] < rank_b[b][cur] {
                match_a[cur] = usize::MAX;
                match_a[a] = b;
                match_b[b] = a;
                queue.push(cur);
                break;
            }
        }
    }
    match_a
}

pub fn matched_pairs(match_a: &[usize]) -> Vec<StablePair> {
    match_a
        .iter()
        .enumerate()
        .filter_map(|(a, &b)| {
            if b == usize::MAX {
                None
            } else {
                Some(StablePair { a, b })
            }
        })
        .collect()
}
