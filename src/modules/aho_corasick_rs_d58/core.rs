use std::collections::VecDeque;

pub fn ac_match_total(patterns: &[&[u8]], text: &[u8]) -> usize {
    #[derive(Clone)]
    struct Node { next: [usize; 26], fail: usize, hits: usize }
    let mut nodes = vec![Node { next: [0; 26], fail: 0, hits: 0 }];
    for &pat in patterns {
        let mut cur = 0usize;
        for &b in pat {
            let c = (b - b'a') as usize;
            if nodes[cur].next[c] == 0 {
                nodes[cur].next[c] = nodes.len();
                nodes.push(Node { next: [0; 26], fail: 0, hits: 0 });
            }
            cur = nodes[cur].next[c];
        }
        nodes[cur].hits += 1;
    }
    let mut q = VecDeque::new();
    for c in 0..26 {
        let nxt = nodes[0].next[c];
        if nxt != 0 { q.push_back(nxt); }
    }
    while let Some(u) = q.pop_front() {
        let f = nodes[u].fail;
        nodes[u].hits += nodes[f].hits;
        for c in 0..26 {
            let v = nodes[u].next[c];
            if v != 0 {
                nodes[v].fail = nodes[f].next[c];
                q.push_back(v);
            } else {
                nodes[u].next[c] = nodes[f].next[c];
            }
        }
    }
    let mut cur = 0usize;
    let mut total = 0usize;
    for &b in text {
        if b < b'a' || b > b'z' { cur = 0; continue; }
        cur = nodes[cur].next[(b - b'a') as usize];
        total += nodes[cur].hits;
    }
    total
}
