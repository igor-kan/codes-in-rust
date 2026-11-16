use crate::errors::TopoError;
use crate::types::TopoOutcome;

pub fn verify_topo_order(n: usize, edges: &[(usize, usize)], out: &TopoOutcome) -> Result<(), TopoError> {
    match out {
        TopoOutcome::Ordered(ord) => {
            if ord.len() != n { return Err(TopoError::InvariantViolation("incomplete ordering")); }
            let mut pos = vec![usize::MAX; n];
            for (idx, &v) in ord.iter().enumerate() {
                if v >= n || pos[v] != usize::MAX { return Err(TopoError::InvariantViolation("duplicate or oob")); }
                pos[v] = idx;
            }
            for &(u, v) in edges {
                if pos[u] >= pos[v] { return Err(TopoError::InvariantViolation("edge ordering violated")); }
            }
            Ok(())
        }
        TopoOutcome::HasCycle(blocked) => {
            if blocked.is_empty() { return Err(TopoError::InvariantViolation("empty cycle witness")); }
            Ok(())
        }
    }
}
