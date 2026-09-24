//! Tests for cuthill_mckee_ordering
#[path = "../src/cuthill_mckee_ordering.rs"]
mod cuthill_mckee_ordering;
use cuthill_mckee_ordering::*;

#[test]
fn test_cuthill_mckee_ordering_execution() {
    assert_eq!(reverse_perm(&[1, 2, 3]), vec![3, 2, 1]);
}
