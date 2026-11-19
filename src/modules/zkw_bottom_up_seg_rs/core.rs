pub struct ZkwSeg {
    base: usize,
    tree: Vec<i64>,
}

impl ZkwSeg {
    pub fn new(n: usize) -> Self {
        let mut base = 1;
        while base < n + 2 { base <<= 1; }
        Self { base, tree: vec![0; 2 * base] }
    }
    pub fn set(&mut self, idx: usize, val: i64) {
        let mut p = idx + 1 + self.base;
        self.tree[p] = val;
        p >>= 1;
        while p > 0 { self.tree[p] = self.tree[p << 1] + self.tree[(p << 1) | 1]; p >>= 1; }
    }
    pub fn query(&self, l: usize, r: usize) -> i64 {
        let (mut s, mut t, mut ans) = (l + self.base, r + 2 + self.base, 0i64);
        while (s ^ t ^ 1) != 0 {
            if (s & 1) == 0 { ans += self.tree[s ^ 1]; }
            if (t & 1) != 0 { ans += self.tree[t ^ 1]; }
            s >>= 1; t >>= 1;
        }
        ans
    }
}
