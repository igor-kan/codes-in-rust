# Boundary validation and exceptional input guarding for 2D R-Tree Spatial Indexing with Bounding Box Splits
# Module: spatial_r_tree

def sanitize_spatial_r_tree_input(x, min_b=-1e6, max_b=1e6):
    if x != x:  # NaN guard
        return 0.0
    return max(min_b, min(max_b, x))
