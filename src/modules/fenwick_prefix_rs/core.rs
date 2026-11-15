use crate::types::FenwickTree;

impl FenwickTree {
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn add(&mut self, idx: usize, delta: i64) {
        self.raw[idx] += delta;
        let n = self.raw.len();
        let mut i = idx + 1;
        while i <= n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    pub fn prefix_sum(&self, count: usize) -> i64 {
        let mut i = count;
        let mut acc = 0i64;
        while i > 0 {
            acc += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        acc
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l >= r {
            return 0;
        }
        self.prefix_sum(r) - self.prefix_sum(l)
    }
}
