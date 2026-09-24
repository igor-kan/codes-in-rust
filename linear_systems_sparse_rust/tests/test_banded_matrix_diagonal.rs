//! Tests for banded_matrix_diagonal
#[path = "../src/banded_matrix_diagonal.rs"]
mod banded_matrix_diagonal;
use banded_matrix_diagonal::*;

#[test]
fn test_banded_matrix_diagonal_execution() {
    assert_eq!(band_bandwidth(1, 1), 3);
}
