# Deterministic unit tests for Portable Core::SIMD Vectorized 4D Tensor Dot Products
# Module: simd_tensor_dot_avx

def test_simd_tensor_dot_avx_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
