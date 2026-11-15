#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::{new_hll, CardinalityEstimator};

#[test]
fn prop_hll_semilattice_commutativity() {
    let mut a = new_hll(10).unwrap();
    let mut b = new_hll(10).unwrap();
    for x in 0..800u64 { a.observe(x); }
    for x in 400..1200u64 { b.observe(x); }
    let mut ab = a.clone();
    ab.merge(&b).unwrap();
    let mut ba = b.clone();
    ba.merge(&a).unwrap();
    assert_eq!(ab.registers, ba.registers);
}
