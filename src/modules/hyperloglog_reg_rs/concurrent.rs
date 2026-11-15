use crate::interface::{new_hll, CardinalityEstimator};
use crate::types::HllSketch;
use std::thread;

pub fn parallel_hll_union(precision: u8, chunks: &[Vec<u64>]) -> HllSketch {
    let mut acc = new_hll(precision).expect("valid p");
    let locals: Vec<HllSketch> = thread::scope(|s| {
        chunks
            .iter()
            .map(|ch| {
                s.spawn(move || {
                    let mut h = new_hll(precision).expect("valid p");
                    for &v in ch {
                        h.observe(v);
                    }
                    h
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|jh| jh.join().expect("thread"))
            .collect()
    });
    for loc in locals {
        acc.merge(&loc).expect("same p");
    }
    acc
}
