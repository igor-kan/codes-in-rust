use crate::types::AhoAutomaton;
use std::thread;

pub fn batch_scan(ac: &AhoAutomaton, corpus: &[&[u8]]) -> Vec<usize> {
    thread::scope(|s| {
        let handles: Vec<_> = corpus.iter().map(|&doc| s.spawn(move || ac.find_all(doc).len())).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}
