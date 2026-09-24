//! Coo Coordinate Format

pub fn coo_entry_valid(r: usize, c: usize, nrows: usize, ncols: usize) -> bool { r < nrows && c < ncols }
