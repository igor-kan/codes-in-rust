//! Tests for csc_matrix_representation
#[path = "../src/csc_matrix_representation.rs"]
mod csc_matrix_representation;
use csc_matrix_representation::*;

#[test]
fn test_csc_matrix_representation_execution() {
    assert_eq!(csc_cols(&[0, 1, 3, 5]), 3);
}
