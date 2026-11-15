use crate::core::find_all_z;
use std::thread;

pub fn parallel_multi_doc_search(docs: &[&[u8]], pattern: &[u8]) -> Vec<Vec<usize>> {
    thread::scope(|s| {
        let handles: Vec<_> = docs.iter().map(|d| s.spawn(move || find_all_z(d, pattern))).collect();
        handles.into_iter().map(|h| h.join().expect("join")).collect()
    })
}
