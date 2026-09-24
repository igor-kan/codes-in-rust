//! Tests for nesterov_accelerated_grad
#[path = "../src/nesterov_accelerated_grad.rs"]
mod nesterov_accelerated_grad;
use nesterov_accelerated_grad::*;

#[test]
fn test_nesterov_accelerated_grad_execution() {
    assert_eq!(nesterov_lookahead(1.0, 2.0, 0.5), 2.0);
}
