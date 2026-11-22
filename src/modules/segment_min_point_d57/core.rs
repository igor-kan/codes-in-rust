pub struct IterSegMin {
    n: usize,
    t: Vec<i64>,
}

impl IterSegMin {
    pub fn from_slice(xs: &[i64]) -> Self {
        let n = xs.len();
        let mut t = vec![i64::MAX; 2 * n];
        t[n..2 * n].copy_from_slice(xs);
        for i in (1..n).rev() {
            t[i] = t[i << 1].min(t[i << 1 | 1]);
        }
        Self { n, t }
    }

    pub fn update(&mut self, mut p: usize, val: i64) {
        p += self.n;
        self.t[p] = val;
        while p > 1 {
            p >>= 1;
            self.t[p] = self.t[p << 1].min(self.t[p << 1 | 1]);
        }
    }

    pub fn query_min(&self, mut l: usize, mut r: usize) -> i64 {
        let mut res = i64::MAX;
        l += self.n;
        r += self.n;
        while l < r {
            if (l & 1) == 1 { res = res.min(self.t[l]); l += 1; }
            if (r & 1) == 1 { r -= 1; res = res.min(self.t[r]); }
            l >>= 1;
            r >>= 1;
        }
        res
    }
}
