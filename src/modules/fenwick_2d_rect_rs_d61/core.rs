use crate::types::RectQuery;

pub struct Fenwick2D {
    rows: usize,
    cols: usize,
    tree: Vec<Vec<i64>>,
}

impl Fenwick2D {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            tree: vec![vec![0i64; cols + 1]; rows + 1],
        }
    }

    pub fn add(&mut self, x: usize, y: usize, delta: i64) {
        let mut i = x + 1;
        while i <= self.rows {
            let mut j = y + 1;
            while j <= self.cols {
                self.tree[i][j] += delta;
                j += j & j.wrapping_neg();
            }
            i += i & i.wrapping_neg();
        }
    }

    fn prefix(&self, x: usize, y: usize) -> i64 {
        let mut sum = 0i64;
        let mut i = x + 1;
        while i > 0 {
            let mut j = y + 1;
            while j > 0 {
                sum += self.tree[i][j];
                j -= j & j.wrapping_neg();
            }
            i -= i & i.wrapping_neg();
        }
        sum
    }

    pub fn rect_sum(&self, q: RectQuery) -> i64 {
        if q.x1 > q.x2 || q.y1 > q.y2 {
            return 0;
        }
        let mut total = self.prefix(q.x2, q.y2);
        if q.x1 > 0 {
            total -= self.prefix(q.x1 - 1, q.y2);
        }
        if q.y1 > 0 {
            total -= self.prefix(q.x2, q.y1 - 1);
        }
        if q.x1 > 0 && q.y1 > 0 {
            total += self.prefix(q.x1 - 1, q.y1 - 1);
        }
        total
    }
}
