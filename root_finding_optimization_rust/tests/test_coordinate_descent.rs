//! Tests for coordinate_descent
#[path = "../src/coordinate_descent.rs"]
mod coordinate_descent;
use coordinate_descent::*;

#[test]
fn test_coordinate_descent_execution() {
    assert_eq!(coordinate_step(3.0, -1.0), 2.0);
}
