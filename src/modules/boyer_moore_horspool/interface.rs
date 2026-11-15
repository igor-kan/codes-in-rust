use crate::errors::BmhError;
use crate::types::HorspoolTable;

pub fn compile_horspool(needle: &[u8]) -> Result<HorspoolTable, BmhError> {
    if needle.is_empty() {
        return Err(BmhError::EmptyInput);
    }
    let m = needle.len();
    let mut shift = [m; 256];
    for i in 0..(m - 1) {
        shift[needle[i] as usize] = m - 1 - i;
    }
    Ok(HorspoolTable { needle: needle.to_vec(), shift })
}
