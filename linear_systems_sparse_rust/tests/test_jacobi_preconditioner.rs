//! Tests for jacobi_preconditioner
#[path = "../src/jacobi_preconditioner.rs"]
mod jacobi_preconditioner;
use jacobi_preconditioner::*;

#[test]
fn test_jacobi_preconditioner_execution() {
    assert_eq!(inv_diagonal(&[2.0, 4.0]), vec![0.5, 0.25]);
}
