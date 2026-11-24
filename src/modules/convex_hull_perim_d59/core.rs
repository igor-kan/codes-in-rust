pub fn hull_area2(mut pts: Vec<(i64, i64)>) -> i64 {
    pts.sort(); pts.dedup();
    if pts.len() < 3 { return 0; }
    let cross = |o: (i64,i64), a: (i64,i64), b: (i64,i64)| -> i64 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    let mut lo = Vec::new();
    for &p in &pts {
        while lo.len() >= 2 && cross(lo[lo.len()-2], lo[lo.len()-1], p) <= 0 { lo.pop(); }
        lo.push(p);
    }
    let mut hi = Vec::new();
    for &p in pts.iter().rev() {
        while hi.len() >= 2 && cross(hi[hi.len()-2], hi[hi.len()-1], p) <= 0 { hi.pop(); }
        hi.push(p);
    }
    lo.pop(); hi.pop(); lo.extend(hi);
    let m = lo.len();
    let mut area2 = 0i64;
    for i in 0..m {
        let a = lo[i]; let b = lo[(i + 1) % m];
        area2 += a.0 * b.1 - a.1 * b.0;
    }
    area2.abs()
}
