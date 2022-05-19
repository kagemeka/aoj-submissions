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
    let r: usize = sc.scan();
    let mut g: Vec<Vec<(usize, u64)>> = vec![vec![]; n];
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let d: u64 = sc.scan();
        g[s].push((t, d));
    }
    let dist = dijkstra_sparse(&g, r);
    for d in dist.iter() {
        if *d < std::u64::MAX {
            writeln!(out, "{}", d).unwrap();
        } else {
            writeln!(out, "INF").unwrap();
        }
    }


}



pub fn dijkstra_sparse(g: &Vec<Vec<(usize, u64)>>, src: usize) -> Vec<u64> {
    let n = g.len();
    let mut dist = vec![u64::MAX; n];
    dist[src] = 0;
    let mut hq = std::collections::BinaryHeap::<(u64, usize)>::new();
    hq.push((0, src));
    while let Some((du, u)) = hq.pop() {
        if du > dist[u] { continue; }
        for (v, w) in g[u].iter() {
            let dv = du + w;
            if dv >= dist[*v] { continue; }
            dist[*v] = dv;
            hq.push((dv, *v));
        }
    }
    dist
}
