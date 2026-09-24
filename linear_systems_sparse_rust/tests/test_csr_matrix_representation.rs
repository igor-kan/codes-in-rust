//! Tests for csr_matrix_representation
#[path = "../src/csr_matrix_representation.rs"]
mod csr_matrix_representation;
use csr_matrix_representation::*;

#[test]
fn test_csr_matrix_representation_execution() {
    assert_eq!(csr_nnz(&[0, 2, 5]), 5);
}
