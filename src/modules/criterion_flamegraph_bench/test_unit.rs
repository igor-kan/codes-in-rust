# Deterministic unit tests for Statistical Microbenchmark Harness with Cycle Metrics
# Module: criterion_flamegraph_bench

def test_criterion_flamegraph_bench_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
