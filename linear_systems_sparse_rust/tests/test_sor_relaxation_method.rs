//! Tests for sor_relaxation_method
#[path = "../src/sor_relaxation_method.rs"]
mod sor_relaxation_method;
use sor_relaxation_method::*;

#[test]
fn test_sor_relaxation_method_execution() {
    assert_eq!(sor_extrapolate(1.0, 2.0, 1.5), 2.5);
}
