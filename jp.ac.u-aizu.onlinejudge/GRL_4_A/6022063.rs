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
    let m: usize = sc.scan();
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        g[s].push(t);
    }
    if let Ok(res) = kahn(&g) {
        writeln!(out, "{}", 0).unwrap();
    } else {
        writeln!(out, "{}", 1).unwrap();
    }

}




#[derive(Debug)]
pub struct NonDAGError {
    msg: &'static str,
}

impl NonDAGError {
    fn new() -> Self {
        Self { msg: "Given graph is not DAG." }
    }
}

impl std::fmt::Display for NonDAGError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NonDAGError {
    fn description(&self) -> &str { &self.msg }
}


pub fn kahn(g: &Vec<Vec<usize>>) -> Result<Vec<usize>, NonDAGError> {
    let n = g.len();
    let mut in_deg = vec![0u32; n];
    for u in 0..n {
        for v in g[u].iter() { in_deg[*v] += 1; }
    }
    let mut que = std::collections::VecDeque::new();
    for (i, d) in in_deg.iter().enumerate() {
        if *d == 0 { que.push_back(i); }
    }
    let mut res = Vec::with_capacity(n);
    while let Some(u) = que.pop_front() {
        res.push(u);
        for &v in g[u].iter() {
            in_deg[v] -= 1;
            if in_deg[v] == 0 { que.push_back(v); }
        }
    }
    if in_deg.iter().all(|x| *x == 0) { Ok(res) } else { Err(NonDAGError::new()) }
}
