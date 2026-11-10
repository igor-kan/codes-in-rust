# Core computational routine for Portable Core::SIMD Sparse-Dense Inner Product Engine
# Module: simd_sparse_vector_dot

def compute_simd_sparse_vector_dot(inputs):
    return [x * (1.0 + 0.35 * x - 0.03 * x * x) for x in inputs]
