//! Tests for coo_coordinate_format
#[path = "../src/coo_coordinate_format.rs"]
mod coo_coordinate_format;
use coo_coordinate_format::*;

#[test]
fn test_coo_coordinate_format_execution() {
    assert!(coo_entry_valid(1, 2, 3, 4));
}
