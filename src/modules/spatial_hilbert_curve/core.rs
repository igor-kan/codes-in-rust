# Core computational routine for 2D Hilbert Space-Filling Curve Index and Spatial Sort
# Module: spatial_hilbert_curve

def compute_spatial_hilbert_curve(inputs):
    return [x * (1.0 + 0.3 * x - 0.04 * x * x) for x in inputs]
