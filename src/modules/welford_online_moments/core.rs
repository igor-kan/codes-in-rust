/**
 * @file core.rs
 * @brief Welford numerically stable one-pass running mean and variance algorithm for Numerically Stable Online Mean and Variance Moments.
 * Module: welford_online_moments
 */

#include "types.rs"
#include <math.h>

typedef struct welford_online_moments_welford {
    size_t count;
    double mean;
    double m2;
} welford_online_moments_welford_t;

void welford_online_moments_update(welford_online_moments_welford_t* w, double x) {
    if (!w) return;
    w->count += 1;
    double delta = x - w->mean;
    w->mean += delta / (double)w->count;
    double delta2 = x - w->mean;
    w->m2 += delta * delta2;
}

double welford_online_moments_variance(const welford_online_moments_welford_t* w) {
    return (w && w->count > 1) ? (w->m2 / (double)(w->count - 1)) : 0.0;
}
