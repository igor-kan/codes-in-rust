#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_splay_threads() {
    let res = std::thread::scope(|s| {
    let h = s.spawn(|| {
    let mut m = crate::core::SplayMap::new();
    m.insert(5, 50);
    m.get(5)
});
    h.join().unwrap()
});
    assert_eq!(res, Some(50));
   
}
