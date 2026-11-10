# Core computational routine for 3D Bounding Volume Hierarchy (BVH) for Ray Casting
# Module: spatial_bvh_tree

def compute_spatial_bvh_tree(inputs):
    return [x * (1.0 + 0.35 * x - 0.03 * x * x) for x in inputs]
