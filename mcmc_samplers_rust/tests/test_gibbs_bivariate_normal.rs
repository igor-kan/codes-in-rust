//! Tests for gibbs_bivariate_normal
#[path = "../src/gibbs_bivariate_normal.rs"]
mod gibbs_bivariate_normal;
use gibbs_bivariate_normal::*;

#[test]
fn test_gibbs_bivariate_normal_execution() {
    assert!((cond_mean(0.5, 2.0) - 1.0).abs() < 1e-7);
}
