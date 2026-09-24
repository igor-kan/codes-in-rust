//! Tests for golden_section_search
#[path = "../src/golden_section_search.rs"]
mod golden_section_search;
use golden_section_search::*;

#[test]
fn test_golden_section_search_execution() {
    assert!(golden_ratio_split(0.0, 1.0) > 0.61);
}
