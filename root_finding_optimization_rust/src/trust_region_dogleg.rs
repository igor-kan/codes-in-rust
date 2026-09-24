//! Trust Region Dogleg

pub fn cauchy_point(grad: f64, hess: f64) -> f64 { -grad / hess }
