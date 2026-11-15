use crate::interface::{build_sketch, FrequencySketch};
use crate::types::{ConservativeSketch, SketchConfig};
use std::thread;

pub fn parallel_shard_merge(cfg: &SketchConfig, shards: &[Vec<u64>]) -> ConservativeSketch {
    let mut merged = build_sketch(cfg).expect("valid config");
    let parts: Vec<ConservativeSketch> = thread::scope(|scope| {
        let mut handles = Vec::new();
        for shard in shards {
            let c = cfg.clone();
            handles.push(scope.spawn(move || {
                let mut local = build_sketch(&c).expect("valid");
                for &k in shard {
                    local.update_conservative(k, 1);
                }
                local
            }));
        }
        handles.into_iter().map(|h| h.join().expect("joined")).collect()
    });
    for p in parts {
        merged.merge_sketch(&p).expect("matching dims");
    }
    merged
}
