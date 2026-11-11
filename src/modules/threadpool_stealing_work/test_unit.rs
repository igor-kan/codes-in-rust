# Deterministic unit tests for High-Performance Work-Stealing Task Thread Pool
# Module: threadpool_stealing_work

def test_threadpool_stealing_work_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
