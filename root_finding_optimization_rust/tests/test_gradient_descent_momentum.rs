//! Tests for gradient_descent_momentum
#[path = "../src/gradient_descent_momentum.rs"]
mod gradient_descent_momentum;
use gradient_descent_momentum::*;

#[test]
fn test_gradient_descent_momentum_execution() {
    assert_eq!(momentum_update(0.0, 0.9, 0.01, 10.0), 0.1);
}
