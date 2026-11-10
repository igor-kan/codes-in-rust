# Boundary validation and exceptional input guarding for Portable Core::SIMD Sparse-Dense Inner Product Engine
# Module: simd_sparse_vector_dot

def sanitize_simd_sparse_vector_dot_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
