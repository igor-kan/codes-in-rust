pub struct SparseMinTable {
    st: Vec<Vec<i64>>,
    lg: Vec<usize>,
}

impl SparseMinTable {
    pub fn build(xs: &[i64]) -> Self {
        let n = xs.len();
        let mut lg = vec![0usize; n + 1];
        for i in 2..=n {
            lg[i] = lg[i / 2] + 1;
        }
        let k = if n > 0 { lg[n] + 1 } else { 1 };
        let mut st = vec![vec![0i64; n]; k];
        st[0].copy_from_slice(xs);
        for j in 1..k {
            let span = 1usize << j;
            let half = 1usize << (j - 1);
            for i in 0..=n.saturating_sub(span) {
                st[j][i] = st[j - 1][i].min(st[j - 1][i + half]);
            }
        }
        Self { st, lg }
    }

    pub fn query_min(&self, l: usize, r: usize) -> i64 {
        let k = self.lg[r - l + 1];
        self.st[k][l].min(self.st[k][r + 1 - (1usize << k)])
    }
}
