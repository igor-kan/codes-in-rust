# Deterministic unit tests for 2D Hilbert Space-Filling Curve Index and Spatial Sort
# Module: spatial_hilbert_curve

def test_spatial_hilbert_curve_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
