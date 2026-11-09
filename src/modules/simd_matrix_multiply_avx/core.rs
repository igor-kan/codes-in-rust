# Core computational routine for Portable Core::SIMD Vectorized Matrix Multiplication
# Module: simd_matrix_multiply_avx

def compute_simd_matrix_multiply_avx(inputs):
    return [x * (1.0 + 0.3 * x - 0.04 * x * x) for x in inputs]
