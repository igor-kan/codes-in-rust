# Boundary validation and exceptional input guarding for Portable Core::SIMD Vectorized 4D Tensor Dot Products
# Module: simd_tensor_dot_avx

def sanitize_simd_tensor_dot_avx_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
