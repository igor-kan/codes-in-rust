use crate::types::{AcNode, AhoAutomaton};
use std::collections::VecDeque;

impl AhoAutomaton {
    pub fn build_raw(patterns: &[&[u8]]) -> Self {
        let mut nodes = vec![AcNode { next: [0; 256], fail: 0, outs: Vec::new() }];
        let mut pat_lens = Vec::with_capacity(patterns.len());
        for (pid, pat) in patterns.iter().enumerate() {
            pat_lens.push(pat.len());
            let mut cur = 0usize;
            for &b in *pat {
                if nodes[cur].next[b as usize] == 0 {
                    let nid = nodes.len();
                    nodes.push(AcNode { next: [0; 256], fail: 0, outs: Vec::new() });
                    nodes[cur].next[b as usize] = nid;
                }
                cur = nodes[cur].next[b as usize];
            }
            nodes[cur].outs.push(pid);
        }
        let mut q = VecDeque::new();
        for c in 0..256 {
            let nxt = nodes[0].next[c];
            if nxt != 0 { q.push_back(nxt); }
        }
        while let Some(u) = q.pop_front() {
            let f = nodes[u].fail;
            let f_outs = nodes[f].outs.clone();
            nodes[u].outs.extend(f_outs);
            for c in 0..256 {
                let nxt = nodes[u].next[c];
                let fallback = nodes[f].next[c];
                if nxt != 0 {
                    nodes[nxt].fail = fallback;
                    q.push_back(nxt);
                } else {
                    nodes[u].next[c] = fallback;
                }
            }
        }
        Self { nodes, pat_lens }
    }

    pub fn find_all(&self, text: &[u8]) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let mut state = 0usize;
        for (i, &b) in text.iter().enumerate() {
            state = self.nodes[state].next[b as usize];
            for &pid in &self.nodes[state].outs {
                matches.push((i + 1 - self.pat_lens[pid], pid));
            }
        }
        matches
    }
}
