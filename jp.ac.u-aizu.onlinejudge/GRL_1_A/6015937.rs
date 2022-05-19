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
    let r: usize = sc.scan();
    let mut g: Vec<Vec<i64>> = vec![vec![inf; n]; n];
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let d: i64 = sc.scan();
        g[s][t] = d;
    }
    let dist = dijkstra_dense(&g, r);
    for d in dist.iter() {
        if *d < std::i64::MAX {
            writeln!(out, "{}", d).unwrap();
        } else {
            writeln!(out, "INF").unwrap();
        }
    }
}


pub fn dijkstra_dense(g: &Vec<Vec<i64>>, src: usize) -> Vec<i64> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    let mut visited = vec![false; n];
    loop {
        let mut u = -1;
        let mut du = inf;
        for i in 0..n {
            if !visited[i] && dist[i] < du { u = i as i32; du = dist[i]; }
        }
        if u == -1 { break; }
        let u = u as usize;
        visited[u] = true;
        for v in 0..n {
            if g[u][v] == inf { continue; }
            let dv = du + g[u][v];
            if dv >= dist[v] { continue; }
            dist[v] = dv;
        }
    }
    dist
}
