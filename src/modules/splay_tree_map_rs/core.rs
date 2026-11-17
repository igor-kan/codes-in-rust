use crate::types::SNode;
pub struct SplayMap { pub nodes: Vec<SNode>, pub root: Option<usize> }
impl SplayMap {
    pub fn new() -> Self { Self { nodes: Vec::new(), root: None } }
    fn rot(&mut self, x: usize) {
        let p = self.nodes[x].parent.unwrap();
        let g = self.nodes[p].parent;
        let d = usize::from(self.nodes[p].ch[1] == Some(x));
        let b = self.nodes[x].ch[d ^ 1];
        if let Some(gi) = g {
            if self.nodes[gi].ch[0] == Some(p) { self.nodes[gi].ch[0] = Some(x); }
            else { self.nodes[gi].ch[1] = Some(x); }
        } else { self.root = Some(x); }
        self.nodes[x].parent = g;
        self.nodes[x].ch[d ^ 1] = Some(p);
        self.nodes[p].parent = Some(x);
        self.nodes[p].ch[d] = b;
        if let Some(bi) = b { self.nodes[bi].parent = Some(p); }
    }
    fn splay(&mut self, x: usize) {
        while let Some(p) = self.nodes[x].parent {
            if let Some(g) = self.nodes[p].parent {
                let zz = (self.nodes[g].ch[0] == Some(p)) == (self.nodes[p].ch[0] == Some(x));
                self.rot(if zz { p } else { x });
            }
            self.rot(x);
        }
    }
    pub fn insert(&mut self, key: i64, val: i64) {
        let mut cur = self.root;
        let mut par = None;
        while let Some(ci) = cur {
            par = Some(ci);
            if key == self.nodes[ci].key { self.nodes[ci].val = val; self.splay(ci); return; }
            cur = self.nodes[ci].ch[usize::from(key > self.nodes[ci].key)];
        }
        let id = self.nodes.len();
        self.nodes.push(SNode { key, val, ch: [None, None], parent: par });
        if let Some(pi) = par {
            let dir = usize::from(key > self.nodes[pi].key);
            self.nodes[pi].ch[dir] = Some(id);
        }
        else { self.root = Some(id); }
        self.splay(id);
    }
    pub fn get(&mut self, key: i64) -> Option<i64> {
        let mut cur = self.root; let mut last = None;
        while let Some(ci) = cur {
            last = Some(ci);
            if key == self.nodes[ci].key { self.splay(ci); return Some(self.nodes[ci].val); }
            cur = self.nodes[ci].ch[usize::from(key > self.nodes[ci].key)];
        }
        if let Some(li) = last { self.splay(li); }
        None
    }
}
