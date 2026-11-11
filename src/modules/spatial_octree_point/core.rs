# Core computational routine for 3D Octree Spatial Indexing with Bounding Frustum Culling
# Module: spatial_octree_point

def compute_spatial_octree_point(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
