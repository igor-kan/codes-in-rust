//! Tests for power_method_sparse
#[path = "../src/power_method_sparse.rs"]
mod power_method_sparse;
use power_method_sparse::*;

#[test]
fn test_power_method_sparse_execution() {
    let mut v = [3.0, 4.0]; normalize_vector(&mut v); assert_eq!(v, [0.6, 0.8]);
}
