use crate::types::Line;

pub struct LiChao {
    dom: usize,
    tree: Vec<Line>,
}

impl LiChao {
    pub fn new(dom: usize) -> Self { Self { dom, tree: vec![Line { m: 0, b: i64::MAX / 4 }; 4 * dom + 4] } }
    fn eval(ln: Line, x: usize) -> i64 { ln.m * (x as i64) + ln.b }
    pub fn add_line(&mut self, m: i64, b: i64) { self.ins(1, 0, self.dom - 1, Line { m, b }); }
    fn ins(&mut self, v: usize, l: usize, r: usize, mut nw: Line) {
        let mid = (l + r) >> 1;
        let lb = Self::eval(nw, l) < Self::eval(self.tree[v], l);
        let mb = Self::eval(nw, mid) < Self::eval(self.tree[v], mid);
        if mb { std::mem::swap(&mut self.tree[v], &mut nw); }
        if l == r { return; }
        if lb != mb { self.ins(v << 1, l, mid, nw); } else { self.ins((v << 1) | 1, mid + 1, r, nw); }
    }
    pub fn query(&self, x: usize) -> i64 {
        let (mut v, mut l, mut r, mut best) = (1, 0, self.dom - 1, i64::MAX / 4);
        loop {
            best = best.min(Self::eval(self.tree[v], x));
            if l == r { return best; }
            let mid = (l + r) >> 1;
            if x <= mid { v <<= 1; r = mid; } else { v = (v << 1) | 1; l = mid + 1; }
        }
    }
}
