//! Tests for gauss_chebyshev_quad
#[path = "../src/gauss_chebyshev_quad.rs"]
mod gauss_chebyshev_quad;
use gauss_chebyshev_quad::*;

#[test]
fn test_gauss_chebyshev_quad_execution() {
    assert!((gc_weight(4) - std::f64::consts::PI / 4.0).abs() < 1e-7);
}
