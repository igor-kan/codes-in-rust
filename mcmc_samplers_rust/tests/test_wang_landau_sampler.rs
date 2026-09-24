//! Tests for wang_landau_sampler
#[path = "../src/wang_landau_sampler.rs"]
mod wang_landau_sampler;
use wang_landau_sampler::*;

#[test]
fn test_wang_landau_sampler_execution() {
    assert_eq!(update_log_dos(2.0, 1.0), 3.0);
}
