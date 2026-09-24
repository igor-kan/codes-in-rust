//! Tests for secant_method_root
#[path = "../src/secant_method_root.rs"]
mod secant_method_root;
use secant_method_root::*;

#[test]
fn test_secant_method_root_execution() {
    assert_eq!(secant_step(2.0, 1.0, 4.0, 1.0), 2.0 - 4.0 / 3.0);
}
