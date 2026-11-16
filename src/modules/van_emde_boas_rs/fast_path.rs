use crate::types::VebTree;

pub fn batch_successors(t: &VebTree, queries: &[usize]) -> Vec<Option<usize>> {
    queries.iter().map(|&q| t.successor(q)).collect()
}
