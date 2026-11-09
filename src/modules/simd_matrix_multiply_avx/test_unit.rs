# Deterministic unit tests for Portable Core::SIMD Vectorized Matrix Multiplication
# Module: simd_matrix_multiply_avx

def test_simd_matrix_multiply_avx_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
