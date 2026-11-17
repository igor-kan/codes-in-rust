#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_wm_oracle() {
    let d = [8u32,3,7,2,9,1,4,6,5];
    let wm = crate::core::WaveletMatrix::build(&d, 4);
    let mut sub = d[2..7].to_vec();
    sub.sort();
    for k in 0..sub.len() {
    assert_eq!(wm.range_kth(2, 7, k), sub[k]);
   
}
}
