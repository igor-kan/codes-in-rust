use crate::core::kahn_min_heap;
use crate::types::TopoOutcome;
use std::thread;

pub fn batch_topo_sort(cases: &[(usize, Vec<(usize, usize)>)]) -> Vec<TopoOutcome> {
    thread::scope(|s| {
        let hs: Vec<_> = cases.iter().map(|(n, e)| s.spawn(move || kahn_min_heap(*n, e))).collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    })
}
