use crate::types::EertreeNode;

pub struct Eertree {
    s: Vec<u8>,
    pub nodes: Vec<EertreeNode>,
    suff: usize,
}

impl Eertree {
    pub fn new(text: &str) -> Self {
        let mut et = Self {
            s: text.as_bytes().to_vec(),
            nodes: vec![
                EertreeNode { len: -1, link: 0, next: [0; 26] },
                EertreeNode { len: 0, link: 0, next: [0; 26] },
            ],
            suff: 1,
        };
        for i in 0..et.s.len() { et.add_pos(i); }
        et
    }
    fn add_pos(&mut self, pos: usize) {
        let mut cur = self.suff;
        let c = (self.s[pos] - b'a') as usize;
        loop {
            let l = self.nodes[cur].len;
            if (pos as isize) - 1 - l >= 0 && self.s[((pos as isize) - 1 - l) as usize] == self.s[pos] { break; }
            cur = self.nodes[cur].link;
        }
        if self.nodes[cur].next[c] != 0 {
            self.suff = self.nodes[cur].next[c];
            return;
        }
        let nw = self.nodes.len();
        let nw_len = self.nodes[cur].len + 2;
        self.nodes.push(EertreeNode { len: nw_len, link: 1, next: [0; 26] });
        self.nodes[cur].next[c] = nw;
        self.suff = nw;
        if nw_len == 1 { return; }
        loop {
            cur = self.nodes[cur].link;
            let l = self.nodes[cur].len;
            if (pos as isize) - 1 - l >= 0 && self.s[((pos as isize) - 1 - l) as usize] == self.s[pos] {
                self.nodes[nw].link = self.nodes[cur].next[c];
                break;
            }
        }
    }
    pub fn distinct_count(&self) -> usize { self.nodes.len() - 2 }
}
