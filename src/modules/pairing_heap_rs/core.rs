use crate::errors::PairingHeapError;
use crate::interface::IndexedMinHeap;
use crate::types::Node;
use crate::validation::validate_handle;

pub struct PairingHeap {
    pub nodes: Vec<Node>,
    pub root: Option<usize>,
    len: usize,
}

impl PairingHeap {
    pub fn new() -> Self { Self { nodes: Vec::new(), root: None, len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }

    fn meld(&mut self, mut a: Option<usize>, mut b: Option<usize>) -> Option<usize> {
        match (a, b) {
            (None, y) => y,
            (x, None) => x,
            (Some(ai), Some(bi)) => {
                if self.nodes[ai].key > self.nodes[bi].key {
                    std::mem::swap(&mut a, &mut b);
                }
                let p = a.unwrap();
                let c = b.unwrap();
                let old_child = self.nodes[p].child;
                self.nodes[c].prev = Some(p);
                self.nodes[c].sibling = old_child;
                if let Some(oc) = old_child {
                    self.nodes[oc].prev = Some(c);
                }
                self.nodes[p].child = Some(c);
                self.nodes[p].prev = None;
                self.nodes[p].sibling = None;
                Some(p)
            }
        }
    }

    fn two_pass(&mut self, mut cur: Option<usize>) -> Option<usize> {
        let mut pairs = Vec::new();
        while let Some(a) = cur {
            let b = self.nodes[a].sibling;
            let nxt = b.and_then(|bi| self.nodes[bi].sibling);
            self.nodes[a].sibling = None;
            self.nodes[a].prev = None;
            if let Some(bi) = b {
                self.nodes[bi].sibling = None;
                self.nodes[bi].prev = None;
                if let Some(m) = self.meld(Some(a), Some(bi)) {
                    pairs.push(m);
                }
            } else {
                pairs.push(a);
            }
            cur = nxt;
        }
        let mut acc = None;
        for m in pairs.into_iter().rev() {
            acc = self.meld(acc, Some(m));
        }
        acc
    }
}

impl IndexedMinHeap for PairingHeap {
    fn push(&mut self, key: i64, val: usize) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node { key, val, child: None, sibling: None, prev: None, active: true });
        self.root = self.meld(self.root, Some(id));
        self.len += 1;
        id
    }

    fn decrease_key(&mut self, id: usize, new_key: i64) -> Result<(), PairingHeapError> {
        validate_handle(&self.nodes, id)?;
        if new_key > self.nodes[id].key {
            return Err(PairingHeapError::InvalidInput("new_key > current key".into()));
        }
        self.nodes[id].key = new_key;
        if self.root == Some(id) {
            return Ok(());
        }
        if let Some(p) = self.nodes[id].prev {
            let sib = self.nodes[id].sibling;
            if self.nodes[p].child == Some(id) {
                self.nodes[p].child = sib;
            } else {
                self.nodes[p].sibling = sib;
            }
            if let Some(s) = sib {
                self.nodes[s].prev = Some(p);
            }
        }
        self.nodes[id].prev = None;
        self.nodes[id].sibling = None;
        self.root = self.meld(self.root, Some(id));
        Ok(())
    }

    fn pop_min(&mut self) -> Option<(i64, usize)> {
        let old = self.root?;
        let res = (self.nodes[old].key, self.nodes[old].val);
        self.nodes[old].active = false;
        let children = self.nodes[old].child.take();
        self.root = self.two_pass(children);
        self.len -= 1;
        Some(res)
    }
}
