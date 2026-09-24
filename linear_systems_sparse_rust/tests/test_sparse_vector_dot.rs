//! Tests for sparse_vector_dot
#[path = "../src/sparse_vector_dot.rs"]
mod sparse_vector_dot;
use sparse_vector_dot::*;

#[test]
fn test_sparse_vector_dot_execution() {
    assert_eq!(sparse_dot(&[1], &[3.0], &[0.0, 2.0]), 6.0);
}
