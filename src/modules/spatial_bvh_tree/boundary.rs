# Boundary validation and exceptional input guarding for 3D Bounding Volume Hierarchy (BVH) for Ray Casting
# Module: spatial_bvh_tree

def sanitize_spatial_bvh_tree_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
