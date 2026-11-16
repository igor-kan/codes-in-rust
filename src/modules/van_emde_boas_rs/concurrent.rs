use crate::types::VebTree;
use std::thread;

pub fn parallel_successor_sum(t: &VebTree, queries: &[usize]) -> usize {
    let mid = queries.len() / 2;
    let (l, r) = queries.split_at(mid);
    thread::scope(|s| {
        let h1 = s.spawn(|| l.iter().filter_map(|&q| t.successor(q)).sum::<usize>());
        let h2 = s.spawn(|| r.iter().filter_map(|&q| t.successor(q)).sum::<usize>());
        h1.join().unwrap() + h2.join().unwrap()
    })
}
