use crate::types::EditStats;

fn classic_distance(a: &[u8], b: &[u8]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for i in 1..=a.len() {
        cur[0] = i;
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            cur[j] = (prev[j] + 1)
                .min(cur[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

pub fn myers_distance(pattern: &str, text: &str) -> usize {
    let p = pattern.as_bytes();
    let t = text.as_bytes();
    let m = p.len();
    if m == 0 {
        return t.len();
    }
    if m > 64 {
        return classic_distance(p, t);
    }
    let mut peq = [0u64; 256];
    for (i, &b) in p.iter().enumerate() {
        peq[b as usize] |= 1u64 << i;
    }
    let mut pv = !0u64;
    let mut mv = 0u64;
    let mut score = m;
    let last = 1u64 << (m - 1);
    for &c in t {
        let eq = peq[c as usize];
        let xv = eq | mv;
        let xh = (((eq & pv).wrapping_add(pv)) ^ pv) | eq;
        let ph = mv | !(xh | pv);
        let mh = pv & xh;
        if ph & last != 0 {
            score += 1;
        }
        if mh & last != 0 {
            score -= 1;
        }
        let ph_shift = (ph << 1) | 1;
        let mh_shift = mh << 1;
        pv = mh_shift | !(xv | ph_shift);
        mv = ph_shift & xv;
    }
    score
}

pub fn edit_stats(pattern: &str, text: &str) -> EditStats {
    EditStats {
        distance: myers_distance(pattern, text),
        pattern_len: pattern.len(),
    }
}
