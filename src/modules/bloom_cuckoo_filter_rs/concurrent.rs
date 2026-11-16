use crate::types::CuckooFilter;
use std::thread;

pub fn parallel_hit_count(cf: &CuckooFilter, keys: &[u64]) -> usize {
    let mid = keys.len() / 2;
    let (a, b) = keys.split_at(mid);
    thread::scope(|s| {
        let h1 = s.spawn(|| a.iter().filter(|&&k| cf.contains(k)).count());
        let h2 = s.spawn(|| b.iter().filter(|&&k| cf.contains(k)).count());
        h1.join().unwrap() + h2.join().unwrap()
    })
}
