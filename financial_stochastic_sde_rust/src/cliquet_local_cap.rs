//! Cliquet Local Cap

pub fn capped_return(r: f64, cap: f64, floor: f64) -> f64 { r.min(cap).max(floor) }
