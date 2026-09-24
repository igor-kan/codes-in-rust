//! Tests for asian_geometric_average
#[path = "../src/asian_geometric_average.rs"]
mod asian_geometric_average;
use asian_geometric_average::*;

#[test]
fn test_asian_geometric_average_execution() {
    assert!((geometric_mean_log(0.0, 5) - 1.0).abs() < 1e-7);
}
