use crate::errors::IntervalError;
use crate::types::Interval;

pub fn coalesce_intervals(raw: &[Interval]) -> Result<Vec<Interval>, IntervalError> {
    for iv in raw {
        if iv.start >= iv.end {
            return Err(IntervalError::InvalidDimension("half-open interval must satisfy start < end"));
        }
    }
    let mut sorted = raw.to_vec();
    sorted.sort_unstable();
    let mut merged: Vec<Interval> = Vec::with_capacity(sorted.len());
    for iv in sorted {
        if let Some(last) = merged.last_mut() {
            if iv.start <= last.end {
                if iv.end > last.end {
                    last.end = iv.end;
                }
                continue;
            }
        }
        merged.push(iv);
    }
    Ok(merged)
}
