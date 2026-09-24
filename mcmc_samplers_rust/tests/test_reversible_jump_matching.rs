//! Tests for reversible_jump_matching
#[path = "../src/reversible_jump_matching.rs"]
mod reversible_jump_matching;
use reversible_jump_matching::*;

#[test]
fn test_reversible_jump_matching_execution() {
    assert_eq!(dimension_jacobian(-2.5), 2.5);
}
