pub fn is_dag_linear(n: usize, edges: &[(usize, usize)]) -> bool {
    matches!(crate::core::kahn_min_heap(n, edges), crate::types::TopoOutcome::Ordered(_))
}
