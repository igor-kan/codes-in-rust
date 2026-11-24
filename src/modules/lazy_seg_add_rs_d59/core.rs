pub struct LazySegTree {
    n: usize,
    t: Vec<i64>,
    lz: Vec<i64>,
}

impl LazySegTree {
    pub fn new(n: usize) -> Self { Self { n, t: vec![0; 4 * n], lz: vec![0; 4 * n] } }

    fn push(&mut self, node: usize, l: usize, r: usize) {
        if self.lz[node] != 0 {
            let mid = (l + r) / 2;
            let v = self.lz[node];
            self.t[node * 2] += v * (mid - l + 1) as i64;
            self.lz[node * 2] += v;
            self.t[node * 2 + 1] += v * (r - mid) as i64;
            self.lz[node * 2 + 1] += v;
            self.lz[node] = 0;
        }
    }

    pub fn add_range(&mut self, ql: usize, qr: usize, v: i64) { self.upd(1, 0, self.n - 1, ql, qr, v); }

    fn upd(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, v: i64) {
        if ql <= l && r <= qr { self.t[node] += v * (r - l + 1) as i64; self.lz[node] += v; return; }
        self.push(node, l, r);
        let mid = (l + r) / 2;
        if ql <= mid { self.upd(node * 2, l, mid, ql, qr, v); }
        if qr > mid { self.upd(node * 2 + 1, mid + 1, r, ql, qr, v); }
        self.t[node] = self.t[node * 2] + self.t[node * 2 + 1];
    }

    pub fn query_sum(&mut self, ql: usize, qr: usize) -> i64 { self.qry(1, 0, self.n - 1, ql, qr) }

    fn qry(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql <= l && r <= qr { return self.t[node]; }
        self.push(node, l, r);
        let mid = (l + r) / 2;
        let mut s = 0i64;
        if ql <= mid { s += self.qry(node * 2, l, mid, ql, qr); }
        if qr > mid { s += self.qry(node * 2 + 1, mid + 1, r, ql, qr); }
        s
    }
}
