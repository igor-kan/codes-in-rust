//! Tests for conjugate_gradient_fletcher
#[path = "../src/conjugate_gradient_fletcher.rs"]
mod conjugate_gradient_fletcher;
use conjugate_gradient_fletcher::*;

#[test]
fn test_conjugate_gradient_fletcher_execution() {
    assert_eq!(fletcher_reeves_beta(2.0, 4.0), 0.5);
}
