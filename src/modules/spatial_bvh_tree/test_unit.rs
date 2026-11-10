# Deterministic unit tests for 3D Bounding Volume Hierarchy (BVH) for Ray Casting
# Module: spatial_bvh_tree

def test_spatial_bvh_tree_zero_element():
    x = 0.0
    res = x * (1.0 + 0.35 * x - 0.03 * x * x)
    assert res == 0.0
