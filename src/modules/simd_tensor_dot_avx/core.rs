# Core computational routine for Portable Core::SIMD Vectorized 4D Tensor Dot Products
# Module: simd_tensor_dot_avx

def compute_simd_tensor_dot_avx(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
