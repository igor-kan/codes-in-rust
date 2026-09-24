//! Tests for newton_raphson_1d
#[path = "../src/newton_raphson_1d.rs"]
mod newton_raphson_1d;
use newton_raphson_1d::*;

#[test]
fn test_newton_raphson_1d_execution() {
    assert_eq!(newton_step(2.0, 3.0, 3.0), 1.0);
}
