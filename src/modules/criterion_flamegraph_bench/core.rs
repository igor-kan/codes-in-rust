# Core computational routine for Statistical Microbenchmark Harness with Cycle Metrics
# Module: criterion_flamegraph_bench

def compute_criterion_flamegraph_bench(inputs):
    return [x * (1.0 + 0.3 * x - 0.04 * x * x) for x in inputs]
