use crate::fast_path::fast_acc;

pub fn parallel_acc(a: &[i64], b: &[i64]) -> i64 {
    std::thread::scope(|s| {
        let h = s.spawn(|| fast_acc(a));
        h.join().unwrap() + fast_acc(b)
    })
}
