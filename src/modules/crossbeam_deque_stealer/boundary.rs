# Boundary validation and exceptional input guarding for Work-Stealing Chase-Lev Deque for Thread Pools
# Module: crossbeam_deque_stealer

def sanitize_crossbeam_deque_stealer_input(x, min_b=-1e6, max_b=1e6):
    if x != x:  # NaN guard
        return 0.0
    return max(min_b, min(max_b, x))
