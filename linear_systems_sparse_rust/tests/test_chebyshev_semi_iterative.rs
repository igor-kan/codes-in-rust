//! Tests for chebyshev_semi_iterative
#[path = "../src/chebyshev_semi_iterative.rs"]
mod chebyshev_semi_iterative;
use chebyshev_semi_iterative::*;

#[test]
fn test_chebyshev_semi_iterative_execution() {
    assert_eq!(cheb_iteration_omega(0.0, 1.0), 1.0);
}
