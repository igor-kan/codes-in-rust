# Core computational routine for Statistical Microbenchmark Harness with Cycle Metrics
# Module: criterion_flamegraph_suite

def compute_criterion_flamegraph_suite(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
