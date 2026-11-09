# Boundary validation and exceptional input guarding for Lock-Free Michael-Scott Queue via Atomic Pointers
# Module: lockfree_queue_michael_scott

def sanitize_lockfree_queue_michael_scott_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
