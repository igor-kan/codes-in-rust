//! Tests for clenshaw_curtis_fft
#[path = "../src/clenshaw_curtis_fft.rs"]
mod clenshaw_curtis_fft;
use clenshaw_curtis_fft::*;

#[test]
fn test_clenshaw_curtis_fft_execution() {
    assert!((cc_chebyshev_node(0, 4) - 1.0).abs() < 1e-7);
}
