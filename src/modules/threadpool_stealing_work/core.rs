# Core computational routine for High-Performance Work-Stealing Task Thread Pool
# Module: threadpool_stealing_work

def compute_threadpool_stealing_work(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
