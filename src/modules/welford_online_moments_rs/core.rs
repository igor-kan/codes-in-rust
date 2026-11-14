/**
 * @file core.rs
 * @brief Numerically stable mean and variance calculation for Numerically Stable Online Calculation of Moments.
 * Module: welford_online_moments_rs
 */

#include "types.rs"

typedef struct welford_online_moments_rs_stat_acc {
    size_t count;
    double mean;
    double m2;
} welford_online_moments_rs_stat_acc_t;

void welford_online_moments_rs_add_sample(welford_online_moments_rs_stat_acc_t* s, double val) {
    if (!s) return;
    s->count += 1;
    double delta = val - s->mean;
    s->mean += delta / (double)s->count;
    double delta2 = val - s->mean;
    s->m2 += delta * delta2;
}

double welford_online_moments_rs_compute_var(const welford_online_moments_rs_stat_acc_t* s) {
    return (s && s->count > 1) ? (s->m2 / (double)(s->count - 1)) : 0.0;
}
