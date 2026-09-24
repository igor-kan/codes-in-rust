//! Gauss Legendre 3Point

pub fn gl3_integral(f_left: f64, f_mid: f64, f_right: f64) -> f64 { (5.0 * f_left + 8.0 * f_mid + 5.0 * f_right) / 9.0 }
