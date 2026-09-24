//! Tests for fibonacci_search_1d
#[path = "../src/fibonacci_search_1d.rs"]
mod fibonacci_search_1d;
use fibonacci_search_1d::*;

#[test]
fn test_fibonacci_search_1d_execution() {
    assert_eq!(fib_ratio(8.0, 13.0), 8.0/13.0);
}
