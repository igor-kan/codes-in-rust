#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use concurrent::parallel_successor_sum;
use fast_path::batch_successors;
use interface::build_veb;
use invariants::verify_veb_oracle;
use std::collections::BTreeSet;

#[test]
fn prop_veb_randomized_oracle() {
    let mut v = build_veb(512).unwrap();
    let mut ref_set = BTreeSet::new();
    let mut rng = 0xCAFEBABE1234u64;
    for step in 0..240 {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = ((rng >> 32) as usize) % 512;
        if step % 5 == 0 {
            v.delete(x); ref_set.remove(&x);
        } else {
            v.insert(x); ref_set.insert(x);
        }
    }
    assert_eq!(verify_veb_oracle(&v, &ref_set), Ok(()));
    let qs: Vec<usize> = (0..512).collect();
    let seq: usize = batch_successors(&v, &qs).into_iter().flatten().sum();
    assert_eq!(parallel_successor_sum(&v, &qs), seq);
}
