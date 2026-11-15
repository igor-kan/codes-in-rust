use crate::interface::from_slice_linear;
use std::thread;

pub fn parallel_range_totals(slices: &[&[i64]], l: usize, r: usize) -> Vec<i64> {
    thread::scope(|s| {
        let handles: Vec<_> = slices
            .iter()
            .map(|sl| s.spawn(move || from_slice_linear(sl).range_sum(l, r)))
            .collect();
        handles.into_iter().map(|h| h.join().expect("ok")).collect()
    })
}
