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
    let mut g: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let d: i64 = sc.scan();
        g[s].push((t, d));
    }
    if let Ok(dist) = bellman_ford_sparse(&g, r) {
        for d in dist.iter() {
            if *d < std::i64::MAX {
                writeln!(out, "{}", d).unwrap();
            } else {
                writeln!(out, "INF").unwrap();
            }
        }
    } else {
        writeln!(out, "NEGATIVE CYCLE").unwrap();
    }
}


#[derive(Debug)]
pub struct NegativeCycleError {
    msg: &'static str,
}

impl NegativeCycleError {
    fn new() -> Self {
        Self { msg: "Negative Cycle Found." }
    }
}

impl std::fmt::Display for NegativeCycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NegativeCycleError {
    fn description(&self) -> &str { &self.msg }
}



pub fn bellman_ford_sparse(g: &Vec<Vec<(usize, i64)>>, src: usize) -> Result<Vec<i64>, NegativeCycleError> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    for _ in 0..n - 1 {
        for u in 0..n {
            for (v, w) in g[u].iter() {
                if dist[u] == inf || dist[u] + w >= dist[*v] { continue; }
                dist[*v] = dist[u] + w;
            }
        }
    }
    for u in 0..n {
        for (v, w) in g[u].iter() {
            if dist[u] == inf || dist[u] + w >= dist[*v] { continue; }
            return Err(NegativeCycleError::new());
        }
    }
    Ok(dist)
}
