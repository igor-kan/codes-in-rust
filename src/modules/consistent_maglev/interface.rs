use crate::errors::MaglevError;
use crate::types::MaglevRouter;

pub fn build_maglev(backends: &[&str], table_size: usize) -> Result<MaglevRouter, MaglevError> {
    if backends.is_empty() {
        return Err(MaglevError::EmptyInput);
    }
    if table_size < backends.len() || !crate::validation::is_prime(table_size) {
        return Err(MaglevError::InvalidDimension("table_size must be prime >= backends.len()"));
    }
    let lookup = crate::core::populate_table(backends, table_size);
    Ok(MaglevRouter {
        table_size,
        backends: backends.iter().map(|s| s.to_string()).collect(),
        lookup,
    })
}
