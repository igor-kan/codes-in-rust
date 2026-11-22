pub fn sam_distinct_substrings(s: &[u8]) -> u64 {
    #[derive(Clone, Default)]
    struct State {
        len: usize,
        link: isize,
        next: Vec<(u8, usize)>,
    }
    let mut st = vec![State { len: 0, link: -1, next: Vec::new() }];
    let mut last = 0usize;
    let get_next = |st: &[State], u: usize, c: u8| -> Option<usize> {
        st[u].next.iter().find(|&&(ch, _)| ch == c).map(|&(_, v)| v)
    };
    let set_next = |st: &mut [State], u: usize, c: u8, v: usize| {
        if let Some(entry) = st[u].next.iter_mut().find(|&&mut (ch, _)| ch == c) {
            entry.1 = v;
        } else {
            st[u].next.push((c, v));
        }
    };
    for &c in s {
        let cur = st.len();
        st.push(State { len: st[last].len + 1, link: 0, next: Vec::new() });
        let mut p = last as isize;
        while p != -1 && get_next(&st, p as usize, c).is_none() {
            set_next(&mut st, p as usize, c, cur);
            p = st[p as usize].link;
        }
        if p == -1 {
            st[cur].link = 0;
        } else {
            let q = get_next(&st, p as usize, c).unwrap();
            if st[p as usize].len + 1 == st[q].len {
                st[cur].link = q as isize;
            } else {
                let clone = st.len();
                let mut cloned = st[q].clone();
                cloned.len = st[p as usize].len + 1;
                st.push(cloned);
                while p != -1 && get_next(&st, p as usize, c) == Some(q) {
                    set_next(&mut st, p as usize, c, clone);
                    p = st[p as usize].link;
                }
                st[q].link = clone as isize;
                st[cur].link = clone as isize;
            }
        }
        last = cur;
    }
    let mut total: u64 = 0;
    for node in st.iter().skip(1) {
        total += (node.len - st[node.link as usize].len) as u64;
    }
    total
}
