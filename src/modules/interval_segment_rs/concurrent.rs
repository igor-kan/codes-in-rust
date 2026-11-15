use crate::core::max_concurrent_overlap;
use crate::types::Interval;
use std::thread;

pub fn parallel_overlap_peaks(batches: &[Vec<Interval>]) -> Vec<usize> {
    thread::scope(|s| {
        let handles: Vec<_> = batches.iter().map(|b| s.spawn(move || max_concurrent_overlap(b))).collect();
        handles.into_iter().map(|h| h.join().expect("ok")).collect()
    })
}
