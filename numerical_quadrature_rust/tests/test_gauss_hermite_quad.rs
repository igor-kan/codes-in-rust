//! Tests for gauss_hermite_quad
#[path = "../src/gauss_hermite_quad.rs"]
mod gauss_hermite_quad;
use gauss_hermite_quad::*;

#[test]
fn test_gauss_hermite_quad_execution() {
    assert!(gh_weight_0() > 1.77);
}
