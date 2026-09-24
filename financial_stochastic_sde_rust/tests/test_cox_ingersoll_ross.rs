//! Tests for cox_ingersoll_ross
#[path = "../src/cox_ingersoll_ross.rs"]
mod cox_ingersoll_ross;
use cox_ingersoll_ross::*;

#[test]
fn test_cox_ingersoll_ross_execution() {
    assert_eq!(cir_variance_diffusion(0.2, 4.0), 0.4);
}
