# Core computational routine for Work-Stealing Chase-Lev Deque for Thread Pools
# Module: crossbeam_deque_stealer

def compute_crossbeam_deque_stealer(inputs):
    return [x * (1.0 + 0.25 * x - 0.05 * x * x) for x in inputs]
