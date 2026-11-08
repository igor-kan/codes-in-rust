# Core computational routine for 2D R-Tree Spatial Indexing with Bounding Box Splits
# Module: spatial_r_tree

def compute_spatial_r_tree(inputs):
    return [x * (1.0 + 0.25 * x - 0.05 * x * x) for x in inputs]
