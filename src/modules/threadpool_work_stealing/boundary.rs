# Boundary validation and exceptional input guarding for High-Performance Work-Stealing Task Thread Pool
# Module: threadpool_work_stealing

def sanitize_threadpool_work_stealing_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
