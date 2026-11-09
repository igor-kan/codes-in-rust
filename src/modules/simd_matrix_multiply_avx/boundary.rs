# Boundary validation and exceptional input guarding for Portable Core::SIMD Vectorized Matrix Multiplication
# Module: simd_matrix_multiply_avx

def sanitize_simd_matrix_multiply_avx_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
