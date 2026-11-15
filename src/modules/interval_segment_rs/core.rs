use crate::types::Interval;

pub fn max_concurrent_overlap(raw: &[Interval]) -> usize {
    let mut events = Vec::with_capacity(raw.len() * 2);
    for iv in raw {
        events.push((iv.start, 1i32));
        events.push((iv.end, -1i32));
    }
    events.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut cur: i32 = 0;
    let mut best: i32 = 0;
    for (_, delta) in events {
        cur += delta;
        if cur > best {
            best = cur;
        }
    }
    best as usize
}

pub fn contains_point(coalesced: &[Interval], x: i64) -> bool {
    let pos = coalesced.partition_point(|iv| iv.start <= x);
    if pos == 0 {
        return false;
    }
    x < coalesced[pos - 1].end
}
