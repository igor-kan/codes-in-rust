//! Tests for triangular_solve_sparse
#[path = "../src/triangular_solve_sparse.rs"]
mod triangular_solve_sparse;
use triangular_solve_sparse::*;

#[test]
fn test_triangular_solve_sparse_execution() {
    assert_eq!(back_subst(8.0, 2.0, 3.0), 2.0);
}
