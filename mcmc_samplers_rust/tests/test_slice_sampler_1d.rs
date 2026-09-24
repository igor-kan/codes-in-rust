//! Tests for slice_sampler_1d
#[path = "../src/slice_sampler_1d.rs"]
mod slice_sampler_1d;
use slice_sampler_1d::*;

#[test]
fn test_slice_sampler_1d_execution() {
    assert_eq!(stepping_out_width(2.0, 5), 10.0);
}
