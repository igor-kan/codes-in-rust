# Boundary validation and exceptional input guarding for 64-Byte Cacheline-Aligned Buffer with SIMD Iterators
# Module: aligned_simd_buffer

def sanitize_aligned_simd_buffer_input(x, min_b=-1e6, max_b=1e6):
    if x != x:  # NaN guard
        return 0.0
    return max(min_b, min(max_b, x))
