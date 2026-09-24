//! Boole Rule Quadrature

pub fn boole_quad(f: &[f64; 5], h: f64) -> f64 { (2.0 * h / 45.0) * (7.0*f[0] + 32.0*f[1] + 12.0*f[2] + 32.0*f[3] + 7.0*f[4]) }
