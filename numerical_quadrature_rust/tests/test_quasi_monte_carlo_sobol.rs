//! Tests for quasi_monte_carlo_sobol
#[path = "../src/quasi_monte_carlo_sobol.rs"]
mod quasi_monte_carlo_sobol;
use quasi_monte_carlo_sobol::*;

#[test]
fn test_quasi_monte_carlo_sobol_execution() {
    assert_eq!(qmc_variance_rate(100.0), 0.01);
}
