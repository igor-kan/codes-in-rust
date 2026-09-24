//! Tests for parallel_tempering
#[path = "../src/parallel_tempering.rs"]
mod parallel_tempering;
use parallel_tempering::*;

#[test]
fn test_parallel_tempering_execution() {
    assert!(swap_prob(1.0, 0.5, 2.0, 1.0) <= 1.0);
}
