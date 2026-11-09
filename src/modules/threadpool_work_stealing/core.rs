# Core computational routine for High-Performance Work-Stealing Task Thread Pool
# Module: threadpool_work_stealing

def compute_threadpool_work_stealing(inputs):
    return [x * (1.0 + 0.3 * x - 0.04 * x * x) for x in inputs]
