use crate::types::LctNode;
pub struct LinkCutForest { pub nodes: Vec<LctNode> }
impl LinkCutForest {
    pub fn new(vals: &[u64]) -> Self {
        Self { nodes: vals.iter().map(|&v| LctNode { p: None, ch: [None, None], rev: false, val: v, xsum: v }).collect() }
    }
    fn is_root(&self, x: usize) -> bool {
        match self.nodes[x].p { None => true, Some(p) => self.nodes[p].ch[0] != Some(x) && self.nodes[p].ch[1] != Some(x) }
    }
    fn push(&mut self, x: usize) {
        if !self.nodes[x].rev { return; }
        self.nodes[x].ch.swap(0, 1);
        for side in 0..2 { if let Some(c) = self.nodes[x].ch[side] { self.nodes[c].rev ^= true; } }
        self.nodes[x].rev = false;
    }
    fn pull(&mut self, x: usize) {
        let mut s = self.nodes[x].val;
        for side in 0..2 { if let Some(c) = self.nodes[x].ch[side] { s ^= self.nodes[c].xsum; } }
        self.nodes[x].xsum = s;
    }
    fn rotate(&mut self, x: usize) {
        let p = self.nodes[x].p.unwrap(); let g = self.nodes[p].p;
        let d = usize::from(self.nodes[p].ch[1] == Some(x)); let b = self.nodes[x].ch[d ^ 1];
        if !self.is_root(p) {
            let gi = g.unwrap();
            if self.nodes[gi].ch[0] == Some(p) { self.nodes[gi].ch[0] = Some(x); }
            else if self.nodes[gi].ch[1] == Some(p) { self.nodes[gi].ch[1] = Some(x); }
        }
        self.nodes[x].p = g; self.nodes[x].ch[d ^ 1] = Some(p);
        self.nodes[p].p = Some(x); self.nodes[p].ch[d] = b;
        if let Some(bi) = b { self.nodes[bi].p = Some(p); }
        self.pull(p); self.pull(x);
    }
    fn splay(&mut self, x: usize) {
        let mut st = vec![x];
        let mut cur = x;
        while !self.is_root(cur) { cur = self.nodes[cur].p.unwrap(); st.push(cur); }
        for idx in st.into_iter().rev() { self.push(idx); }
        while !self.is_root(x) {
            let p = self.nodes[x].p.unwrap();
            if !self.is_root(p) {
                let g = self.nodes[p].p.unwrap();
                let zz = (self.nodes[p].ch[0] == Some(x)) == (self.nodes[g].ch[0] == Some(p));
                self.rotate(if zz { p } else { x });
            }
            self.rotate(x);
        }
    }
    fn access(&mut self, x: usize) {
        let mut last = None; let mut y = Some(x);
        while let Some(yi) = y {
            self.splay(yi); self.nodes[yi].ch[1] = last; self.pull(yi);
            last = Some(yi); y = self.nodes[yi].p;
        }
        self.splay(x);
    }
    pub fn make_root(&mut self, u: usize) { self.access(u); self.nodes[u].rev ^= true; self.push(u); }
    pub fn find_root(&mut self, mut u: usize) -> usize {
        self.access(u);
        while let Some(l) = self.nodes[u].ch[0] { self.push(u); u = l; }
        self.splay(u); u
    }
    pub fn link(&mut self, u: usize, v: usize) -> bool {
        self.make_root(u);
        if self.find_root(v) == u { return false; }
        self.nodes[u].p = Some(v); true
    }
    pub fn cut(&mut self, u: usize, v: usize) -> bool {
        self.make_root(u);
        if self.find_root(v) != u || self.nodes[v].p != Some(u) || self.nodes[v].ch[0].is_some() { return false; }
        self.nodes[v].p = None; self.nodes[u].ch[1] = None; self.pull(u); true
    }
    pub fn path_xor(&mut self, u: usize, v: usize) -> u64 { self.make_root(u); self.access(v); self.nodes[v].xsum }
}
