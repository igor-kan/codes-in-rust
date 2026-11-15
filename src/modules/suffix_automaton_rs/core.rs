use crate::types::{SamState, SuffixAutomaton};
use std::collections::BTreeMap;

impl SuffixAutomaton {
    pub fn extend(&mut self, ch: u8) {
        let cur = self.states.len();
        let new_len = self.states[self.last].len + 1;
        self.states.push(SamState { len: new_len, link: Some(0), next: BTreeMap::new() });
        self.input_len += 1;

        let mut p_opt = Some(self.last);
        while let Some(p) = p_opt {
            if self.states[p].next.contains_key(&ch) {
                break;
            }
            self.states[p].next.insert(ch, cur);
            p_opt = self.states[p].link;
        }

        if let Some(p) = p_opt {
            let q = self.states[p].next[&ch];
            if self.states[p].len + 1 == self.states[q].len {
                self.states[cur].link = Some(q);
            } else {
                let clone = self.states.len();
                let mut q_clone = self.states[q].clone();
                q_clone.len = self.states[p].len + 1;
                self.states.push(q_clone);
                let mut pp_opt = Some(p);
                while let Some(pp) = pp_opt {
                    if self.states[pp].next.get(&ch) == Some(&q) {
                        self.states[pp].next.insert(ch, clone);
                        pp_opt = self.states[pp].link;
                    } else {
                        break;
                    }
                }
                self.states[q].link = Some(clone);
                self.states[cur].link = Some(clone);
            }
        }
        self.last = cur;
    }

    pub fn contains_substring(&self, sub: &[u8]) -> bool {
        let mut st = 0usize;
        for &b in sub {
            match self.states[st].next.get(&b) {
                Some(&nxt) => st = nxt,
                None => return false,
            }
        }
        true
    }

    pub fn distinct_substrings(&self) -> u64 {
        let mut total = 0u64;
        for st in &self.states[1..] {
            let link_len = st.link.map(|li| self.states[li].len).unwrap_or(0);
            total += (st.len - link_len) as u64;
        }
        total
    }
}
