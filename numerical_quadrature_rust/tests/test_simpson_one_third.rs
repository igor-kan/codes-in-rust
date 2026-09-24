//! Tests for simpson_one_third
#[path = "../src/simpson_one_third.rs"]
mod simpson_one_third;
use simpson_one_third::*;

#[test]
fn test_simpson_one_third_execution() {
    assert!((simpson13(0.0, 1.0, 4.0, 1.0) - 8.0/3.0).abs() < 1e-7);
}
