# Core computational routine for 64-Byte Cacheline-Aligned Buffer with SIMD Iterators
# Module: aligned_simd_buffer

def compute_aligned_simd_buffer(inputs):
    return [x * (1.0 + 0.25 * x - 0.05 * x * x) for x in inputs]
