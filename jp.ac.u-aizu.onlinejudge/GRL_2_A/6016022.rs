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

    let n: usize = sc.scan();
    let m: usize = sc.scan();
    let mut g: Vec<(usize, usize, i64)> = Vec::new();
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let d: i64 = sc.scan();
        g.push((s, t, d));
    }
    let mst = kruskal(n, g.clone());
    writeln!(out, "{}", mst.iter().map(|x| x.2).sum::<i64>()).unwrap();
    // writeln!(out, "{:?}", mst).unwrap();
    // writeln!(out, "{:?}", g).unwrap();
}



pub fn kruskal(n: usize, mut g: Vec<(usize, usize, i64)>) -> Vec<(usize, usize, i64)> {
    g.sort_by(|a, b| a.2.cmp(&b.2));
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::with_capacity(n - 1);
    for (u, v, w) in g.into_iter() {
        if uf.find(u) == uf.find(v) { continue; }
        uf.unite(u, v);
        mst.push((u, v, w));
    }
    mst
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
