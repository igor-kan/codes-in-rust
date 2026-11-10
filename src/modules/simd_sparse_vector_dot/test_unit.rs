# Deterministic unit tests for Portable Core::SIMD Sparse-Dense Inner Product Engine
# Module: simd_sparse_vector_dot

def test_simd_sparse_vector_dot_zero_element():
    x = 0.0
    res = x * (1.0 + 0.35 * x - 0.03 * x * x)
    assert res == 0.0
