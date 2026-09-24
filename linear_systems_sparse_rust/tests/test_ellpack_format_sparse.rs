//! Tests for ellpack_format_sparse
#[path = "../src/ellpack_format_sparse.rs"]
mod ellpack_format_sparse;
use ellpack_format_sparse::*;

#[test]
fn test_ellpack_format_sparse_execution() {
    assert_eq!(ellpack_index(2, 1, 4), 9);
}
