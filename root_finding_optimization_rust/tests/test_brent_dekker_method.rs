//! Tests for brent_dekker_method
#[path = "../src/brent_dekker_method.rs"]
mod brent_dekker_method;
use brent_dekker_method::*;

#[test]
fn test_brent_dekker_method_execution() {
    assert_eq!(brent_inverse_quadratic(1.0, 2.0, 3.0), 2.0);
}
