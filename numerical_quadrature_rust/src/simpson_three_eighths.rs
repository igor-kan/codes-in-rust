//! Simpson Three Eighths

pub fn simpson38(f0: f64, f1: f64, f2: f64, f3: f64, h: f64) -> f64 { (3.0 * h / 8.0) * (f0 + 3.0*f1 + 3.0*f2 + f3) }
