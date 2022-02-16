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

    // let inf = std::i64::MAX;
    let n: usize = sc.scan();
    let mut g: Vec<(usize, usize)> = Vec::with_capacity(n - 1);
    for u in 0..n {
        let k: usize = sc.scan();
        for _ in 0..k {
            let v: usize = sc.scan();
            g.push((u, v));
        }
    }

    let q: usize = sc.scan();
    let mut uv = Vec::with_capacity(q);
    for _ in 0..q {
        let u: usize = sc.scan();
        let v: usize = sc.scan();
        uv.push((u, v));
    }
    for x in tarjan_offline(&g, &uv, 0) {
        writeln!(out, "{}", x).unwrap();
    }
}


pub struct UnionFind {
    data: Vec<i32>,
}


impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { data: vec![-1; n] }
    }
    
    pub fn find(&mut self, u: usize) -> usize {
        if self.data[u] < 0 { return u; }
        self.data[u] = self.find(self.data[u] as usize) as i32;
        self.data[u] as usize
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let (mut u, mut v) = (self.find(u), self.find(v));
        if u == v { return; }
        if self.data[u] > self.data[v] { std::mem::swap(&mut u, &mut v); }
        self.data[u] += self.data[v];
        self.data[v] = u as i32;
    }

    pub fn size(&mut self, u: usize) -> usize {
        let u = self.find(u);
        -self.data[u] as usize
    }
}


/// Lowest Common Ancestor with Tarjan's offline algorithm.
/// O(V + Q) preprocessing, O(1) per query.
/// references
/// - https://cp-algorithms.com/graph/lca_tarjan.html 
/// - https://en.wikipedia.org/wiki/Tarjan%27s_off-line_lowest_common_ancestors_algorithm
/// - https://tjkendev.github.io/procon-library/python/graph/lca-tarjan.html
pub fn tarjan_offline(g: &Vec<(usize, usize)>, uv: &Vec<(usize, usize)>, root: usize) -> Vec<usize> {
    fn dfs(
        g: &Vec<Vec<usize>>, 
        q: &Vec<Vec<(usize, usize)>>, 
        visited: &mut Vec<bool>, 
        uf: &mut UnionFind, 
        ancestor: &mut Vec<usize>, 
        lca: &mut Vec<usize>,
        u: usize,
    ) {
        visited[u] = true;
        ancestor[u] = u;
        for &v in g[u].iter() {
            if visited[v] { continue; }            
            dfs(g, q, visited, uf, ancestor, lca, v);
            uf.unite(u, v);
            ancestor[uf.find(u)] = u;
        }
        for &(v, i) in q[u].iter() {
            if visited[v] { lca[i] = ancestor[uf.find(v)]; }
        }
    }
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for &(u, v) in g.iter() {
        t[u].push(v);
        t[v].push(u);
    }
    let mut q = vec![vec![]; n];
    for (i, &(u, v)) in uv.iter().enumerate() {
        q[u].push((v, i));
        q[v].push((u, i));
    }
    let mut visited = vec![false; n];
    let mut uf = UnionFind::new(n);
    let mut ancestor = vec![n; n];
    let mut lca = vec![n; uv.len()];
    dfs(&t, &q, &mut visited, &mut uf, &mut ancestor, &mut lca, root);
    lca
}

