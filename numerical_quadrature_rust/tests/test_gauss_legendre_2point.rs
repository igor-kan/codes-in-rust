//! Tests for gauss_legendre_2point
#[path = "../src/gauss_legendre_2point.rs"]
mod gauss_legendre_2point;
use gauss_legendre_2point::*;

#[test]
fn test_gauss_legendre_2point_execution() {
    assert_eq!(gl2_integral(1.5, 2.5), 4.0);
}
