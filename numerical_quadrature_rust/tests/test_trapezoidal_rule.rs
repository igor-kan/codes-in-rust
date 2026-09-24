//! Tests for trapezoidal_rule
#[path = "../src/trapezoidal_rule.rs"]
mod trapezoidal_rule;
use trapezoidal_rule::*;

#[test]
fn test_trapezoidal_rule_execution() {
    assert_eq!(trapezoid(2.0, 4.0, 1.0), 3.0);
}
