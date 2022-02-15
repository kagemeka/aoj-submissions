pub struct Scanner<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    /// let stdin = std::io::stdin();
    /// let mut sc = Scanner::new(stdin.lock());
    pub fn new(reader: R) -> Self { Self { reader: reader } }

    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
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
    // let m: usize = sc.scan();
    let mut g: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    for _ in 0..n - 1 {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let w: i64 = sc.scan();
        g.push((s, t, w));
    }
    let (path, diameter) = tree_diameter(&g);
    writeln!(out, "{}", diameter).unwrap();
}



pub fn tree_diameter(g: &Vec<(usize, usize, i64)>) -> (Vec<usize>, i64) {
    fn dfs(g: &Vec<Vec<(usize, i64)>>, root: usize) -> (Vec<usize>, Vec<i64>) {
        let mut st = vec![root];
        let n = g.len();
        let mut parent = vec![n; n];
        let mut dist = vec![0; n];
        while let Some(u) = st.pop() {
            for &(v, w) in g[u].iter() {
                if v == parent[u] { continue; }
                parent[v] = u;
                dist[v] = dist[u] + w;
                st.push(v);
            }
        }
        (parent, dist)
    }
    let n = g.len() + 1;
    let mut t: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, w) in g {
        t[u].push((v, w));
        t[v].push((u, w));
    }
    let (_, dist) = dfs(&t, 0);
    let u = dist.iter().enumerate().max_by_key(|(_, &d)| d).unwrap().0;
    let (parent, dist) = dfs(&t, u);
    let (mut v, &diameter) = dist.iter().enumerate().max_by_key(|&(v, d)| (d, v)).unwrap();
    let mut path = Vec::new();
    while v != n {
        path.push(v);
        v = parent[v];
    }
    (path, diameter)
}
