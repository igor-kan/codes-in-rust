//! Tests for gauss_laguerre_quad
#[path = "../src/gauss_laguerre_quad.rs"]
mod gauss_laguerre_quad;
use gauss_laguerre_quad::*;

#[test]
fn test_gauss_laguerre_quad_execution() {
    assert_eq!(gl_node_1(), 1.0);
}
