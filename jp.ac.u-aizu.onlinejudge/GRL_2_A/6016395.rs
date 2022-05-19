pub struct Scanner<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    /// let stdin = std::io::stdin();
    /// let mut sc = Scanner::new(stdin.lock());
    pub fn new(reader: R) -> Self { Self { reader: reader } }

    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c|c.unwrap()as char)
        .skip_while(|c|c.is_whitespace())
        .take_while(|c|!c.is_whitespace())
        .collect::<String>().parse::<T>().ok().unwrap()
    }
}



// #[allow(warnings)]
fn main() {
    use std::io::Write;
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(std::io::BufReader::new(stdin.lock()));
    let stdout = std::io::stdout();
    let out = &mut std::io::BufWriter::new(stdout.lock());

    let inf = std::i64::MAX;
    let n: usize = sc.scan();
    let m: usize = sc.scan();
    // let mut g: Vec<(usize, usize, i64)> = Vec::with_capacity(m);
    let mut g: Vec<Vec<i64>> = vec![vec![inf; n]; n];
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let d: i64 = sc.scan();
        g[s][t] = d;
        g[t][s] = d;
        // g.push((s, t, d));
    }
    let mst = prim_dense(&g);
    writeln!(out, "{}", mst.iter().map(|x| x.2).sum::<i64>()).unwrap();
}




pub fn prim_dense(g: &Vec<Vec<i64>>) -> Vec<(usize, usize, i64)> {
    let n = g.len();
    for u in 1..n {
        for v in 0..u { assert_eq!(g[u][v], g[v][u]); }
    }
    let inf = std::i64::MAX;
    let mut mst: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    let mut min_edge = vec![(n, inf); n];
    min_edge[0] = (0, 0);
    let mut visited = vec![false; n];
    for _ in 0..n {
        let mut pre = n;
        let mut u = n;
        let mut wu = inf;
        for i in 0..n {
            if visited[i] || min_edge[i].1 >= wu { continue; }
            u = i;
            pre = min_edge[i].0;
            wu = min_edge[i].1
        }
        assert!(wu < inf);
        visited[u] = true;
        if pre != u { mst.push((pre, u, wu)); }
        for v in 0..n {
            if visited[v] || g[u][v] >= min_edge[v].1 { continue; }
            min_edge[v] = (u, g[u][v]);
        }
    }
    assert_eq!(mst.len(), n - 1);
    mst
}



pub fn prim_sparse(n: usize, g: &Vec<(usize, usize, i64)>) -> Vec<(usize, usize, i64)> {
    let mut t: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for (u, v, w) in g.iter() {
        t[*u].push((*v, *w));
        t[*v].push((*u, *w));
    }
    let mut mst: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    let mut hq = std::collections::BinaryHeap::new();
    hq.push((0, 0, 0));
    let inf = std::i64::MAX;
    let mut weight = vec![inf; n];
    let mut visited = vec![false; n];
    while let Some((wu, pre, u)) = hq.pop() {
        if visited[u] { continue; }
        visited[u] = true;
        if pre != u { mst.push((pre, u, -wu)); }
        for (v, wv) in t[u].iter() {
            if visited[*v] || *wv >= weight[*v] { continue; }
            weight[*v] = *wv;
            hq.push((-wv, u, *v));
        }
    }
    mst
}
