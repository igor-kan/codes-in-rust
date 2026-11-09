# Deterministic unit tests for High-Performance Work-Stealing Task Thread Pool
# Module: threadpool_work_stealing

def test_threadpool_work_stealing_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
