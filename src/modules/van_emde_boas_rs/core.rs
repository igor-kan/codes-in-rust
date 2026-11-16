use crate::types::VebTree;

impl VebTree {
    pub fn new_unchecked(u: usize) -> Self {
        if u <= 2 {
            return Self { u: 2, shift: 1, mask: 1, min: None, max: None, summary: None, clusters: Vec::new() };
        }
        let bits = u.trailing_zeros();
        let lower_bits = bits / 2;
        let upper_bits = bits - lower_bits;
        let lo_size = 1usize << lower_bits;
        let hi_size = 1usize << upper_bits;
        Self {
            u,
            shift: lower_bits,
            mask: lo_size - 1,
            min: None,
            max: None,
            summary: Some(Box::new(Self::new_unchecked(hi_size))),
            clusters: (0..hi_size).map(|_| Self::new_unchecked(lo_size)).collect(),
        }
    }

    #[inline] fn high(&self, x: usize) -> usize { x >> self.shift }
    #[inline] fn low(&self, x: usize) -> usize { x & self.mask }
    #[inline] fn index(&self, hi: usize, lo: usize) -> usize { (hi << self.shift) | lo }

    pub fn contains(&self, x: usize) -> bool {
        if Some(x) == self.min || Some(x) == self.max {
            return true;
        }
        if self.u == 2 {
            return false;
        }
        self.clusters[self.high(x)].contains(self.low(x))
    }

    pub fn insert(&mut self, mut x: usize) {
        if self.min.is_none() {
            self.min = Some(x);
            self.max = Some(x);
            return;
        }
        let cur_min = self.min.unwrap();
        if x == cur_min || Some(x) == self.max {
            return;
        }
        if x < cur_min {
            std::mem::swap(&mut x, self.min.as_mut().unwrap());
        }
        if self.u > 2 {
            let hi = self.high(x);
            let lo = self.low(x);
            if self.clusters[hi].min.is_none() {
                self.summary.as_mut().unwrap().insert(hi);
                self.clusters[hi].min = Some(lo);
                self.clusters[hi].max = Some(lo);
            } else {
                self.clusters[hi].insert(lo);
            }
        }
        if x > self.max.unwrap() {
            self.max = Some(x);
        }
    }

    pub fn successor(&self, x: usize) -> Option<usize> {
        if self.u == 2 {
            if x == 0 && self.max == Some(1) { return Some(1); }
            return None;
        }
        if let Some(m) = self.min {
            if x < m { return Some(m); }
        }
        let hi = self.high(x);
        let lo = self.low(x);
        if let Some(c_max) = self.clusters[hi].max {
            if lo < c_max {
                let offset = self.clusters[hi].successor(lo)?;
                return Some(self.index(hi, offset));
            }
        }
        let succ_cluster = self.summary.as_ref()?.successor(hi)?;
        let offset = self.clusters[succ_cluster].min?;
        Some(self.index(succ_cluster, offset))
    }

    pub fn delete(&mut self, mut x: usize) {
        if self.min.is_none() { return; }
        if self.min == self.max {
            if self.min == Some(x) {
                self.min = None;
                self.max = None;
            }
            return;
        }
        if self.u == 2 {
            if x == 0 && self.min == Some(0) {
                self.min = Some(1);
                self.max = Some(1);
            } else if x == 1 && self.max == Some(1) {
                self.min = Some(0);
                self.max = Some(0);
            }
            return;
        }
        if Some(x) == self.min {
            let first_cluster = match self.summary.as_ref().unwrap().min {
                Some(c) => c,
                None => return,
            };
            x = self.index(first_cluster, self.clusters[first_cluster].min.unwrap());
            self.min = Some(x);
        }
        let hi = self.high(x);
        let lo = self.low(x);
        self.clusters[hi].delete(lo);
        if self.clusters[hi].min.is_none() {
            self.summary.as_mut().unwrap().delete(hi);
            if Some(x) == self.max {
                if let Some(s_max) = self.summary.as_ref().unwrap().max {
                    self.max = Some(self.index(s_max, self.clusters[s_max].max.unwrap()));
                } else {
                    self.max = self.min;
                }
            }
        } else if Some(x) == self.max {
            self.max = Some(self.index(hi, self.clusters[hi].max.unwrap()));
        }
    }
}
