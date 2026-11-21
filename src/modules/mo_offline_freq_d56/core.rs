pub fn distinct_in_ranges(xs: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    if xs.is_empty() || queries.is_empty() {
        return Vec::new();
    }
    let blk = (xs.len() as f64).sqrt().max(1.0) as usize;
    let mut qs: Vec<(usize, usize, usize)> = queries.iter().enumerate().map(|(i, &(l, r))| (l, r, i)).collect();
    qs.sort_by_key(|&(l, r, _)| {
        let b = l / blk;
        (b, if b % 2 == 0 { r } else { usize::MAX - r })
    });
    let max_val = *xs.iter().max().unwrap_or(&0);
    let mut freq = vec![0usize; max_val + 1];
    let mut cur_distinct = 0usize;
    let mut ans = vec![0usize; queries.len()];
    let mut cl: usize = 0;
    let mut cr: isize = -1;
    let add = |pos: usize, freq: &mut [usize], d: &mut usize| {
        if freq[xs[pos]] == 0 { *d += 1; }
        freq[xs[pos]] += 1;
    };
    let rem = |pos: usize, freq: &mut [usize], d: &mut usize| {
        freq[xs[pos]] -= 1;
        if freq[xs[pos]] == 0 { *d -= 1; }
    };
    for (ql, qr, qidx) in qs {
        while cr < qr as isize {
            cr += 1;
            add(cr as usize, &mut freq, &mut cur_distinct);
        }
        while cl > ql {
            cl -= 1;
            add(cl, &mut freq, &mut cur_distinct);
        }
        while cr > qr as isize {
            rem(cr as usize, &mut freq, &mut cur_distinct);
            cr -= 1;
        }
        while cl < ql {
            rem(cl, &mut freq, &mut cur_distinct);
            cl += 1;
        }
        ans[qidx] = cur_distinct;
    }
    ans
}
