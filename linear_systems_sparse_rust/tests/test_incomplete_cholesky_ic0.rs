//! Tests for incomplete_cholesky_ic0
#[path = "../src/incomplete_cholesky_ic0.rs"]
mod incomplete_cholesky_ic0;
use incomplete_cholesky_ic0::*;

#[test]
fn test_incomplete_cholesky_ic0_execution() {
    assert_eq!(ic0_pivot(10.0, 1.0), 3.0);
}
