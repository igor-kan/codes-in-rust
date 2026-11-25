#[derive(Clone, Debug)]
pub enum RopeNode {
    Leaf(String),
    Branch { weight: usize, total: usize, left: Box<RopeNode>, right: Box<RopeNode> },
}

impl RopeNode {
    pub fn from_str(s: &str) -> Self { Self::Leaf(s.to_owned()) }
    pub fn len(&self) -> usize {
        match self { Self::Leaf(s) => s.len(), Self::Branch { total, .. } => *total }
    }
    pub fn concat(left: RopeNode, right: RopeNode) -> Self {
        let wl = left.len();
        let tot = wl + right.len();
        Self::Branch { weight: wl, total: tot, left: Box::new(left), right: Box::new(right) }
    }
    pub fn byte_at(&self, idx: usize) -> Option<u8> {
        match self {
            Self::Leaf(s) => s.as_bytes().get(idx).copied(),
            Self::Branch { weight, left, right, .. } => {
                if idx < *weight { left.byte_at(idx) } else { right.byte_at(idx - *weight) }
            }
        }
    }
    pub fn collect_into(&self, out: &mut String) {
        match self {
            Self::Leaf(s) => out.push_str(s),
            Self::Branch { left, right, .. } => { left.collect_into(out); right.collect_into(out); }
        }
    }
    pub fn split(self, idx: usize) -> (Self, Self) {
        match self {
            Self::Leaf(s) => {
                let pos = idx.min(s.len());
                (Self::Leaf(s[..pos].to_owned()), Self::Leaf(s[pos..].to_owned()))
            }
            Self::Branch { weight, left, right, .. } => {
                if idx <= weight {
                    let (ll, lr) = left.split(idx);
                    (ll, Self::concat(lr, *right))
                } else {
                    let (rl, rr) = right.split(idx - weight);
                    (Self::concat(*left, rl), rr)
                }
            }
        }
    }
}
