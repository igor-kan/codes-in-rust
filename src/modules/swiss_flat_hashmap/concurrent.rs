use crate::interface::FlatDict;
use crate::types::SwissFlatMap;
use std::thread;

pub fn parallel_read_checksum(map: &SwissFlatMap, keys: &[u64]) -> i64 {
    let mid = keys.len() / 2;
    let (left, right) = keys.split_at(mid);
    thread::scope(|s| {
        let h1 = s.spawn(|| left.iter().filter_map(|&k| map.get(k)).sum::<i64>());
        let h2 = s.spawn(|| right.iter().filter_map(|&k| map.get(k)).sum::<i64>());
        h1.join().unwrap() + h2.join().unwrap()
    })
}
