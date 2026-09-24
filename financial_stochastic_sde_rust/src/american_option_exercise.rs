//! American Option Exercise

pub fn early_exercise(hold_val: f64, payoff: f64) -> f64 { hold_val.max(payoff) }
