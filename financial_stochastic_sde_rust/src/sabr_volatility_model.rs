//! Sabr Volatility Model

pub fn sabr_backbone(f: f64, beta: f64) -> f64 { f.powf(beta) }
