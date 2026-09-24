//! Tests for romberg_extrapolation
#[path = "../src/romberg_extrapolation.rs"]
mod romberg_extrapolation;
use romberg_extrapolation::*;

#[test]
fn test_romberg_extrapolation_execution() {
    assert_eq!(richardson(4.0, 1.0, 1), 5.0);
}
