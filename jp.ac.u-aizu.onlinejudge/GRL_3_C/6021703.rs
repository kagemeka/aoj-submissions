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
    let label = kosaraju(&g);
    let q: usize = sc.scan();
    for _ in 0..q {
        let u: usize = sc.scan();
        let v: usize = sc.scan();
        writeln!(out, "{:?}", if label[u] == label[v] { 1 } else { 0 }).unwrap();
    }
}


pub fn kosaraju(g: &Vec<Vec<usize>>) -> Vec<usize> {
    fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, que: &mut Vec<usize>, u: usize) {
        visited[u] = true;
        for v in g[u].iter() {
            if !visited[*v] { dfs(g, visited, que, *v); }
        }
        que.push(u);
    }
    fn rev_dfs(g: &Vec<Vec<usize>>, label: &mut Vec<usize>, l: usize, u: usize) {
        label[u] = l;
        for v in g[u].iter() {
            if label[*v] == g.len() { rev_dfs(g, label, l, *v); }
        }
    }
    let n = g.len();
    let mut visited = vec![false; n];
    let mut que = Vec::with_capacity(n);
    let mut label = vec![n; n];
    let mut l = 0usize;
    for i in 0..n {
        if !visited[i] { dfs(g, &mut visited, &mut que, i); }
    }
    let mut t = vec![vec![]; n];
    for u in 0..n {
        for v in g[u].iter() { t[*v].push(u); }
    }
    for i in que.iter().rev() {
        if label[*i] != n { continue; }
        rev_dfs(&t, &mut label, l, *i);
        l += 1;
    }
    label
}
