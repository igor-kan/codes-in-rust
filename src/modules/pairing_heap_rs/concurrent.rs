use crate::fast_path::fast_heapsort;

pub fn parallel_sort_slices(chunks: &[Vec<i64>]) -> Vec<Vec<i64>> {
    std::thread::scope(|s| {
        let handles: Vec<_> = chunks.iter().map(|c| s.spawn(|| fast_heapsort(c))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}
