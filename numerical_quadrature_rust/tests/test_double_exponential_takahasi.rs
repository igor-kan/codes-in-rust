//! Tests for double_exponential_takahasi
#[path = "../src/double_exponential_takahasi.rs"]
mod double_exponential_takahasi;
use double_exponential_takahasi::*;

#[test]
fn test_double_exponential_takahasi_execution() {
    assert!(tanh_sinh_weight(0.0) > 0.0);
}
