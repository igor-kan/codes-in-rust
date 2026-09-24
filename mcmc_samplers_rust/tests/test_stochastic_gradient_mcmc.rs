//! Tests for stochastic_gradient_mcmc
#[path = "../src/stochastic_gradient_mcmc.rs"]
mod stochastic_gradient_mcmc;
use stochastic_gradient_mcmc::*;

#[test]
fn test_stochastic_gradient_mcmc_execution() {
    assert_eq!(sgld_step(1.0, 0.1, 2.0), 0.9);
}
