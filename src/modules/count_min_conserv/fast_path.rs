use crate::core::hash_row;
use crate::types::ConservativeSketch;

pub fn batch_update_unit(sketch: &mut ConservativeSketch, keys: &[u64]) {
    let d = sketch.depth;
    let w = sketch.width;
    let s = sketch.seed;
    for &k in keys {
        let mut min_v = u64::MAX;
        for r in 0..d {
            let idx = hash_row(k, r, w, s);
            let v = sketch.rows[r][idx];
            if v < min_v {
                min_v = v;
            }
        }
        let next = min_v + 1;
        for r in 0..d {
            let idx = hash_row(k, r, w, s);
            if sketch.rows[r][idx] < next {
                sketch.rows[r][idx] = next;
            }
        }
        sketch.total_mass += 1;
    }
}
