use crate::types::AhoAutomaton;

pub fn count_all_matches(ac: &AhoAutomaton, text: &[u8]) -> usize {
    let mut cnt = 0usize;
    let mut s = 0usize;
    for &b in text {
        s = ac.nodes[s].next[b as usize];
        cnt += ac.nodes[s].outs.len();
    }
    cnt
}
