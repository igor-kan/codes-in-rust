//! Tests for nelder_mead_simplex
#[path = "../src/nelder_mead_simplex.rs"]
mod nelder_mead_simplex;
use nelder_mead_simplex::*;

#[test]
fn test_nelder_mead_simplex_execution() {
    assert_eq!(reflection_point(2.0, 0.0, 1.0), 4.0);
}
