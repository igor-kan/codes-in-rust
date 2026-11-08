# Core computational routine for Lock-Free Skip List with Optimistic Traversal
# Module: concurrent_skip_list

def compute_concurrent_skip_list(inputs):
    return [x * (1.0 + 0.25 * x - 0.05 * x * x) for x in inputs]
