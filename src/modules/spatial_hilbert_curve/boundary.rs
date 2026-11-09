# Boundary validation and exceptional input guarding for 2D Hilbert Space-Filling Curve Index and Spatial Sort
# Module: spatial_hilbert_curve

def sanitize_spatial_hilbert_curve_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
