use crate::errors::TopoError;
use crate::types::TopoOutcome;

pub fn lex_topo_sort(n: usize, edges: &[(usize, usize)]) -> Result<TopoOutcome, TopoError> {
    if n == 0 {
        return Err(TopoError::InvalidInput("zero vertices"));
    }
    for &(u, v) in edges {
        if u >= n || v >= n {
            return Err(TopoError::OutOfBounds("edge endpoint >= n"));
        }
    }
    Ok(crate::core::kahn_min_heap(n, edges))
}
