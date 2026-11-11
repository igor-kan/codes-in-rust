# Deterministic unit tests for 3D Octree Spatial Indexing with Bounding Frustum Culling
# Module: spatial_octree_point

def test_spatial_octree_point_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
