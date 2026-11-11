# Core computational routine for Lock-Free Freelist using Hazard Pointers for Memory
# Module: lockfree_freelist_hazard

def compute_lockfree_freelist_hazard(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
