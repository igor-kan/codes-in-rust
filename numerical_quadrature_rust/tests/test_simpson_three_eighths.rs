//! Tests for simpson_three_eighths
#[path = "../src/simpson_three_eighths.rs"]
mod simpson_three_eighths;
use simpson_three_eighths::*;

#[test]
fn test_simpson_three_eighths_execution() {
    assert!(simpson38(1.0, 1.0, 1.0, 1.0, 1.0) > 0.0);
}
