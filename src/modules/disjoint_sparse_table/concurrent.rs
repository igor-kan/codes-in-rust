use crate::types::DisjointSparseTable;
use std::thread;

pub fn parallel_query_xor(t: &DisjointSparseTable, qs: &[(usize, usize)]) -> u64 {
    let mid = qs.len() / 2;
    let (a, b) = qs.split_at(mid);
    thread::scope(|s| {
        let h1 = s.spawn(|| a.iter().fold(0u64, |acc, &(l, r)| acc ^ t.query_inclusive(l, r)));
        let h2 = s.spawn(|| b.iter().fold(0u64, |acc, &(l, r)| acc ^ t.query_inclusive(l, r)));
        h1.join().unwrap() ^ h2.join().unwrap()
    })
}
