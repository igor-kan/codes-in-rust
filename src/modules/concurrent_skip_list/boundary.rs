# Boundary validation and exceptional input guarding for Lock-Free Skip List with Optimistic Traversal
# Module: concurrent_skip_list

def sanitize_concurrent_skip_list_input(x, min_b=-1e6, max_b=1e6):
    if x != x:  # NaN guard
        return 0.0
    return max(min_b, min(max_b, x))
