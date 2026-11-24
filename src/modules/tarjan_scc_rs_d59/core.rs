pub fn tarjan_scc(n: usize, edges: &[(usize, usize)]) -> (usize, Vec<usize>) {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges { adj[u].push(v); }
    struct Ctx { tin: Vec<isize>, low: Vec<isize>, on: Vec<bool>, stk: Vec<usize>, comp: Vec<usize>, t: isize, c: usize }
    let mut ctx = Ctx { tin: vec![-1; n], low: vec![-1; n], on: vec![false; n], stk: Vec::new(), comp: vec![0; n], t: 0, c: 0 };
    fn dfs(u: usize, adj: &[Vec<usize>], ctx: &mut Ctx) {
        ctx.tin[u] = ctx.t; ctx.low[u] = ctx.t; ctx.t += 1;
        ctx.stk.push(u); ctx.on[u] = true;
        for &v in &adj[u] {
            if ctx.tin[v] == -1 {
                dfs(v, adj, ctx);
                ctx.low[u] = ctx.low[u].min(ctx.low[v]);
            } else if ctx.on[v] {
                ctx.low[u] = ctx.low[u].min(ctx.tin[v]);
            }
        }
        if ctx.low[u] == ctx.tin[u] {
            while let Some(x) = ctx.stk.pop() {
                ctx.on[x] = false;
                ctx.comp[x] = ctx.c;
                if x == u { break; }
            }
            ctx.c += 1;
        }
    }
    for i in 0..n { if ctx.tin[i] == -1 { dfs(i, &adj, &mut ctx); } }
    (ctx.c, ctx.comp)
}
