# Boundary validation and exceptional input guarding for Cacheline-Padded Bounded SPSC Ring Buffer Queue
# Module: lockfree_spsc_ring_bounded

def sanitize_lockfree_spsc_ring_bounded_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
