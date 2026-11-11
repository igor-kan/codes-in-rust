# Boundary validation and exceptional input guarding for Lock-Free Freelist using Hazard Pointers for Memory
# Module: lockfree_freelist_hazard

def sanitize_lockfree_freelist_hazard_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
