//! Tests for halley_cubic_root
#[path = "../src/halley_cubic_root.rs"]
mod halley_cubic_root;
use halley_cubic_root::*;

#[test]
fn test_halley_cubic_root_execution() {
    assert_eq!(halley_step(1.0, 0.0, 1.0, 0.0), 1.0);
}
