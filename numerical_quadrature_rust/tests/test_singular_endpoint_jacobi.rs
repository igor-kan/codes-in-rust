//! Tests for singular_endpoint_jacobi
#[path = "../src/singular_endpoint_jacobi.rs"]
mod singular_endpoint_jacobi;
use singular_endpoint_jacobi::*;

#[test]
fn test_singular_endpoint_jacobi_execution() {
    assert_eq!(jacobi_alpha_beta_weight(0.0, 0.5, 0.5), 1.0);
}
