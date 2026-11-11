# Deterministic unit tests for Statistical Microbenchmark Harness with Cycle Metrics
# Module: criterion_flamegraph_suite

def test_criterion_flamegraph_suite_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
