/**
 * @file core.rs
 * @brief Welford numerically stable one-pass running mean and variance algorithm for Numerically Stable Online Calculation of Moments.
 * Module: welford_online_accumulator
 */

#include "types.rs"

typedef struct welford_online_accumulator_accumulator {
    size_t count;
    double mean;
    double m2;
} welford_online_accumulator_accumulator_t;

void welford_online_accumulator_push(welford_online_accumulator_accumulator_t* acc, double val) {
    if (!acc) return;
    acc->count += 1;
    double delta = val - acc->mean;
    acc->mean += delta / (double)acc->count;
    double delta2 = val - acc->mean;
    acc->m2 += delta * delta2;
}

double welford_online_accumulator_get_variance(const welford_online_accumulator_accumulator_t* acc) {
    return (acc && acc->count > 1) ? (acc->m2 / (double)(acc->count - 1)) : 0.0;
}
