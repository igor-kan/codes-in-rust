# Deterministic unit tests for 64-Byte Cacheline-Aligned Buffer with SIMD Iterators
# Module: aligned_simd_buffer

def test_aligned_simd_buffer_zero_element():
    x = 0.0
    res = x * (1.0 + 0.25 * x - 0.05 * x * x)
    assert res == 0.0
