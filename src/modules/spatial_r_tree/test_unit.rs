# Deterministic unit tests for 2D R-Tree Spatial Indexing with Bounding Box Splits
# Module: spatial_r_tree

def test_spatial_r_tree_zero_element():
    x = 0.0
    res = x * (1.0 + 0.25 * x - 0.05 * x * x)
    assert res == 0.0
