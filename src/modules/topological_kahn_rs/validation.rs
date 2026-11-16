use crate::errors::TopoError;

pub fn validate_dag_dims(n: usize, edges: &[(usize, usize)]) -> Result<(), TopoError> {
    if n == 0 { return Err(TopoError::InvalidInput("empty")); }
    for &(u, v) in edges {
        if u >= n || v >= n { return Err(TopoError::OutOfBounds("vertex out of range")); }
    }
    Ok(())
}
