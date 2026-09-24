//! Tests for american_option_exercise
#[path = "../src/american_option_exercise.rs"]
mod american_option_exercise;
use american_option_exercise::*;

#[test]
fn test_american_option_exercise_execution() {
    assert_eq!(early_exercise(5.0, 7.0), 7.0);
}
