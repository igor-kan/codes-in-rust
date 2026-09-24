//! Sor Relaxation Method

pub fn sor_extrapolate(x_old: f64, x_gs: f64, omega: f64) -> f64 { (1.0 - omega) * x_old + omega * x_gs }
