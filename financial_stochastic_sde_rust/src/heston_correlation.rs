//! Heston Correlation

pub fn correlated_brownian(z1: f64, z2: f64, rho: f64) -> f64 { rho * z1 + (1.0 - rho*rho).sqrt() * z2 }
