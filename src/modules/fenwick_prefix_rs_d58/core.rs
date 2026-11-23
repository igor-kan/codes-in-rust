pub struct Fenwick {
    bit: Vec<i64>,
}

impl Fenwick {
    pub fn new(n: usize) -> Self { Self { bit: vec![0; n + 1] } }

    pub fn add(&mut self, idx: usize, delta: i64) {
        let mut i = idx + 1;
        while i < self.bit.len() {
            self.bit[i] += delta;
            i += i & (!i + 1);
        }
    }

    pub fn prefix(&self, idx: usize) -> i64 {
        let mut s = 0i64;
        let mut i = idx + 1;
        while i > 0 {
            s += self.bit[i];
            i -= i & (!i + 1);
        }
        s
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix(r) - if l > 0 { self.prefix(l - 1) } else { 0 }
    }
}
