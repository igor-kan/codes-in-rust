//! Tests for adaptive_quadrature_kronrod
#[path = "../src/adaptive_quadrature_kronrod.rs"]
mod adaptive_quadrature_kronrod;
use adaptive_quadrature_kronrod::*;

#[test]
fn test_adaptive_quadrature_kronrod_execution() {
    assert_eq!(kronrod_error(2.0, 2.01), 0.010000000000000231);
}
