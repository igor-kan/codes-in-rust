use crate::types::SuffixAutomaton;

pub fn longest_common_substring_len(sam: &SuffixAutomaton, query: &[u8]) -> usize {
    let mut v = 0usize;
    let mut l = 0usize;
    let mut best = 0usize;
    for &c in query {
        while v != 0 && !sam.states[v].next.contains_key(&c) {
            v = sam.states[v].link.unwrap_or(0);
            l = sam.states[v].len;
        }
        if let Some(&nxt) = sam.states[v].next.get(&c) {
            v = nxt;
            l += 1;
        }
        if l > best {
            best = l;
        }
    }
    best
}
