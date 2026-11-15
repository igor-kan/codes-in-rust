use crate::types::MaglevRouter;

pub fn backend_slot_counts(router: &MaglevRouter) -> Vec<usize> {
    let mut counts = vec![0usize; router.backends.len()];
    for &bid in &router.lookup {
        counts[bid] += 1;
    }
    counts
}
