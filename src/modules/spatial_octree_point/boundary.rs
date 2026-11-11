# Boundary validation and exceptional input guarding for 3D Octree Spatial Indexing with Bounding Frustum Culling
# Module: spatial_octree_point

def sanitize_spatial_octree_point_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
