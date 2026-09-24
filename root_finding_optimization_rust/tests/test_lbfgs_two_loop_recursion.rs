//! Tests for lbfgs_two_loop_recursion
#[path = "../src/lbfgs_two_loop_recursion.rs"]
mod lbfgs_two_loop_recursion;
use lbfgs_two_loop_recursion::*;

#[test]
fn test_lbfgs_two_loop_recursion_execution() {
    assert_eq!(lbfgs_alpha(2.0, 4.0), 2.0);
}
