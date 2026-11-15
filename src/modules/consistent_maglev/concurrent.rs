use crate::types::MaglevRouter;
use std::thread;

pub fn parallel_route_keys(router: &MaglevRouter, keys: &[u64]) -> Vec<usize> {
    thread::scope(|s| {
        let mid = keys.len() / 2;
        let (a, b) = keys.split_at(mid);
        let h1 = s.spawn(|| a.iter().map(|&k| router.route_key(k)).collect::<Vec<_>>());
        let h2 = s.spawn(|| b.iter().map(|&k| router.route_key(k)).collect::<Vec<_>>());
        let mut out = h1.join().expect("ok");
        out.extend(h2.join().expect("ok"));
        out
    })
}
