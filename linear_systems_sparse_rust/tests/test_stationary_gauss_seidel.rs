//! Tests for stationary_gauss_seidel
#[path = "../src/stationary_gauss_seidel.rs"]
mod stationary_gauss_seidel;
use stationary_gauss_seidel::*;

#[test]
fn test_stationary_gauss_seidel_execution() {
    assert_eq!(gs_forward_step(10.0, 4.0, 2.0), 3.0);
}
