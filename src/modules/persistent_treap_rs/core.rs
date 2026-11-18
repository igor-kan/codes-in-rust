use crate::types::PTNode;

pub struct PersistentTreap {
    pub pool: Vec<PTNode>,
}

impl PersistentTreap {
    pub fn new() -> Self { Self { pool: Vec::new() } }
    pub fn sz(&self, u: Option<usize>) -> usize { u.map_or(0, |i| self.pool[i].sz) }
    pub fn sum(&self, u: Option<usize>) -> i64 { u.map_or(0, |i| self.pool[i].sum) }
    pub fn make(&mut self, val: i64, prio: u32, l: Option<usize>, r: Option<usize>) -> usize {
        let sz = 1 + self.sz(l) + self.sz(r);
        let sum = val + self.sum(l) + self.sum(r);
        self.pool.push(PTNode { val, sum, prio, sz, l, r });
        self.pool.len() - 1
    }
    pub fn split(&mut self, u: Option<usize>, k: usize) -> (Option<usize>, Option<usize>) {
        let Some(idx) = u else { return (None, None); };
        let nd = self.pool[idx];
        let lsz = self.sz(nd.l);
        if k <= lsz {
            let (ll, lr) = self.split(nd.l, k);
            let nr = self.make(nd.val, nd.prio, lr, nd.r);
            (ll, Some(nr))
        } else {
            let (rl, rr) = self.split(nd.r, k - lsz - 1);
            let nl = self.make(nd.val, nd.prio, nd.l, rl);
            (Some(nl), rr)
        }
    }
    pub fn merge(&mut self, a: Option<usize>, b: Option<usize>) -> Option<usize> {
        match (a, b) {
            (None, y) => y,
            (x, None) => x,
            (Some(ai), Some(bi)) => {
                let na = self.pool[ai]; let nb = self.pool[bi];
                if na.prio > nb.prio {
                    let nr = self.merge(na.r, Some(bi));
                    Some(self.make(na.val, na.prio, na.l, nr))
                } else {
                    let nl = self.merge(Some(ai), nb.l);
                    Some(self.make(nb.val, nb.prio, nl, nb.r))
                }
            }
        }
    }
}
