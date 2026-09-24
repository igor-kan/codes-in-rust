//! Tests for bisection_bracket
#[path = "../src/bisection_bracket.rs"]
mod bisection_bracket;
use bisection_bracket::*;

#[test]
fn test_bisection_bracket_execution() {
    assert_eq!(bisect_mid(1.0, 3.0), 2.0);
}
