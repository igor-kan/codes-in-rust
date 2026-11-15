#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use core::contains_point;
use interface::coalesce_intervals;
use invariants::verify_canonical_disjoint;
use types::Interval;

#[test]
fn prop_point_membership_preserved() {
    let raw = vec![
        Interval { start: 2, end: 10 },
        Interval { start: 5, end: 15 },
        Interval { start: 20, end: 25 },
        Interval { start: 22, end: 30 },
    ];
    let merged = coalesce_intervals(&raw).unwrap();
    assert_eq!(verify_canonical_disjoint(&merged), Ok(()));
    for x in 0..35 {
        let brute = raw.iter().any(|iv| iv.start <= x && x < iv.end);
        assert_eq!(contains_point(&merged, x), brute);
    }
}
