use crate::interface::build_sam;
use std::thread;

pub fn parallel_distinct_counts(inputs: &[&[u8]]) -> Vec<u64> {
    thread::scope(|s| {
        let handles: Vec<_> = inputs
            .iter()
            .map(|inp| s.spawn(move || build_sam(inp).expect("ok").distinct_substrings()))
            .collect();
        handles.into_iter().map(|h| h.join().expect("ok")).collect()
    })
}
