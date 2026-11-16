use crate::core::compute_manacher;
use std::thread;

pub fn batch_longest_lengths(inputs: &[&[u8]]) -> Vec<usize> {
    thread::scope(|s| {
        let hs: Vec<_> = inputs.iter().map(|&inp| s.spawn(move || compute_manacher(inp).longest_len)).collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    })
}
