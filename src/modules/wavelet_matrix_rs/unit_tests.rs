#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_wm_kth() {
    let d = [3u32,1,4,1,5,9,2,6];
    let wm = crate::core::WaveletMatrix::build(&d, 4);
    assert_eq!(wm.range_kth(1, 6, 2), 4);
    assert_eq!(wm.range_kth(0, 8, 7), 9);
   
}
