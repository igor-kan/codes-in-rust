//! Vasicek Yield Curve

pub fn zero_coupon_yield(p: f64, t: f64) -> f64 { -p.ln() / t }
