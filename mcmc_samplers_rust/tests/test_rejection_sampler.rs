//! Tests for rejection_sampler
#[path = "../src/rejection_sampler.rs"]
mod rejection_sampler;
use rejection_sampler::*;

#[test]
fn test_rejection_sampler_execution() {
    assert!(rejection_bound(0.5, 0.6, 1.0));
}
