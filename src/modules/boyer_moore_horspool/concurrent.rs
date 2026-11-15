use crate::types::HorspoolTable;
use std::thread;

pub fn parallel_find_in_chunks(tbl: &HorspoolTable, chunks: &[&[u8]]) -> Vec<Vec<usize>> {
    thread::scope(|s| {
        let hs: Vec<_> = chunks.iter().map(|c| s.spawn(move || tbl.find_all(c))).collect();
        hs.into_iter().map(|h| h.join().expect("ok")).collect()
    })
}
