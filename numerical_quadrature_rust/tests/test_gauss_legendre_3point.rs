//! Tests for gauss_legendre_3point
#[path = "../src/gauss_legendre_3point.rs"]
mod gauss_legendre_3point;
use gauss_legendre_3point::*;

#[test]
fn test_gauss_legendre_3point_execution() {
    assert!((gl3_integral(1.0, 1.0, 1.0) - 2.0).abs() < 1e-7);
}
