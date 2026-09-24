//! Simpson One Third

pub fn simpson13(f_a: f64, f_m: f64, f_b: f64, h: f64) -> f64 { (h / 3.0) * (f_a + 4.0 * f_m + f_b) }
