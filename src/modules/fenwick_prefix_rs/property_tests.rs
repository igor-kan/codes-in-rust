#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::from_slice_linear;
use invariants::verify_all_prefixes;

#[test]
fn prop_fenwick_random_updates_exact() {
    let mut bit = from_slice_linear(&vec![1i64; 64]);
    for step in 0..200usize {
        let idx = (step * 19 + 7) % 64;
        let delta = ((step % 11) as i64) - 5;
        bit.add(idx, delta);
        assert_eq!(verify_all_prefixes(&bit), Ok(()));
    }
}
