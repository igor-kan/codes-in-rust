//! Tests for bfgs_quasi_newton
#[path = "../src/bfgs_quasi_newton.rs"]
mod bfgs_quasi_newton;
use bfgs_quasi_newton::*;

#[test]
fn test_bfgs_quasi_newton_execution() {
    assert_eq!(bfgs_damping(1.5), 1.0);
}
