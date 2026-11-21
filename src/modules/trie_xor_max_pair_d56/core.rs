pub fn max_xor_pair(nums: &[u32]) -> u32 {
    if nums.len() < 2 {
        return 0;
    }
    let mut nodes: Vec<[usize; 2]> = vec![[0, 0]];
    let mut best = 0u32;
    for (idx, &num) in nums.iter().enumerate() {
        if idx > 0 {
            let mut cur = 0usize;
            let mut x = 0u32;
            for b in (0..31).rev() {
                let bit = ((num >> b) & 1) as usize;
                let want = 1 ^ bit;
                if nodes[cur][want] != 0 {
                    x |= 1u32 << b;
                    cur = nodes[cur][want];
                } else {
                    cur = nodes[cur][bit];
                }
            }
            best = best.max(x);
        }
        let mut cur = 0usize;
        for b in (0..31).rev() {
            let bit = ((num >> b) & 1) as usize;
            if nodes[cur][bit] == 0 {
                nodes[cur][bit] = nodes.len();
                nodes.push([0, 0]);
            }
            cur = nodes[cur][bit];
        }
    }
    best
}
