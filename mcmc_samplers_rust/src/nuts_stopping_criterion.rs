//! Nuts Stopping Criterion

pub fn u_turn_detected(p_sharp_theta: f64) -> bool { p_sharp_theta < 0.0 }
