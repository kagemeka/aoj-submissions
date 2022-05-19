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
    // let lca = BinaryLifting::new(&g, 0);
    let q: usize = sc.scan();
    // for _ in 0..q {
    //     let u: usize = sc.scan();
    //     let v: usize = sc.scan();
    //     writeln!(out, "{}", lca.get(u, v)).unwrap();
    // }
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



pub fn tree_bfs(g: &Vec<(usize, usize)>, root: usize) -> (Vec<usize>, Vec<usize>) {
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for &(u, v) in g.iter() {
        t[u].push(v);
        t[v].push(u);
    }
    let mut parent = vec![n; n];
    let mut depth = vec![0; n];
    let mut que = std::collections::VecDeque::new();
    que.push_back(root);
    while let Some(u) = que.pop_front() {
        for &v in t[u].iter() {
            if v == parent[u] { continue; }
            parent[v] = u;
            depth[v] = depth[u] + 1;
            que.push_back(v);
        }
    }
    (parent, depth)
}



pub fn bit_length(n: usize) -> usize {
    let mut l = 0usize;
    while 1 << l <= n { l += 1; }
    l
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
/// O(N + M)
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
    let m = uv.len();

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
    let mut lca = vec![n; m];
    dfs(&t, &q, &mut visited, &mut uf, &mut ancestor, &mut lca, root);
    lca
}



/// Lowest Common Ancestor with Binary Lifting.
/// references
/// - https://cp-algorithms.com/graph/lca_binary_lifting.html
pub struct BinaryLifting {
    ancestor: Vec<Vec<usize>>,
    depth: Vec<usize>,
}


impl BinaryLifting {
    /// O(N\log{N})
    pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
        let n = g.len() + 1;
        let (parent, depth) = tree_bfs(g, root);
        let k = std::cmp::max(1, bit_length(*depth.iter().max().unwrap()));
        let mut ancestor = vec![vec![n; n]; k];
        ancestor[0] = parent;
        ancestor[0][root] = root;
        for i in 0..k - 1 {
            for j in 0..n {
                ancestor[i + 1][j] = ancestor[i][ancestor[i][j]];
            }
        }
        Self { ancestor: ancestor, depth: depth }
    }

    /// O(\log{N})
    pub fn get(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] { std::mem::swap(&mut u, &mut v); }
        let d = self.depth[v] - self.depth[u];
        for i in 0..bit_length(d) {
            if d >> i & 1 == 1 { v = self.ancestor[i][v]; }
        }
        if v == u { return u; }
        for a in self.ancestor.iter().rev() {
            let nu = a[u];
            let nv = a[v];
            if nu != nv { u = nu; v = nv;}
        }
        self.ancestor[0][u]
    }
}



// Fn(&S, &S) -> S is a trait.
/// this is a dynamic size object at compilation time.
/// thus, it's needed to be enclosed with Box<dyn> pointer.
pub struct Monoid<S> {
    pub op: Box<dyn Fn(&S, &S) -> S>,
    pub e: Box<dyn Fn() -> S>,
    pub commutative: bool,
}
