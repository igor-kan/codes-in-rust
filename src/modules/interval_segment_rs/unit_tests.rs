#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use core::{contains_point, max_concurrent_overlap};
use fast_path::total_covered_length;
use interface::coalesce_intervals;
use invariants::verify_canonical_disjoint;
use types::Interval;

#[test]
fn test_coalesce_and_sweep() {
    let raw = [
        Interval { start: 1, end: 5 },
        Interval { start: 3, end: 8 },
        Interval { start: 10, end: 12 },
        Interval { start: 8, end: 9 },
    ];
    let merged = coalesce_intervals(&raw).unwrap();
    assert_eq!(merged, vec![Interval { start: 1, end: 9 }, Interval { start: 10, end: 12 }]);
    assert_eq!(verify_canonical_disjoint(&merged), Ok(()));
    assert_eq!(total_covered_length(&merged), 10);
    assert_eq!(max_concurrent_overlap(&raw), 2);
    assert!(contains_point(&merged, 4));
    assert!(!contains_point(&merged, 9));
}
