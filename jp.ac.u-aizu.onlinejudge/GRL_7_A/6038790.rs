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


    let size_a: usize = sc.scan();
    let size_b: usize = sc.scan();
    let m: usize = sc.scan();
    let n = size_a + size_b;
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = sc.scan();
        let v: usize = sc.scan();
        g[u].push(v + size_a);
        g[v + size_a].push(u);
    }
    let max_match = ford_fulkerson(size_a, size_b, &g);
    writeln!(out, "{}", max_match).unwrap();
}


/// O(VE)
/// references
/// - https://en.wikipedia.org/wiki/Maximum_cardinality_matching
/// - https://ei1333.github.io/luzhiled/snippets/graph/bipartite-matching.html
/// - https://onlinejudge.u-aizu.ac.jp/solutions/problem/GRL_7_A/review/5630283/ngtkana/Rust
/// - https://onlinejudge.u-aizu.ac.jp/solutions/problem/GRL_7_A/review/4329190/sansen/Rust
pub fn ford_fulkerson(size_a: usize, size_b: usize, g: &[Vec<usize>]) -> usize {
    fn dfs(g: &[Vec<usize>], pair: &mut [Option<usize>], visited: &mut [bool], u: usize) -> bool {
        visited[u] = true;
        for &v in g[u].iter() {
            if !pair[v].map_or(true, |v| !visited[v] && dfs(g, pair, visited, v)) { continue; }
            pair[v] = Some(u);
            pair[u] = Some(v);
            return true;
        }
        false
    }
    let n = g.len();
    assert_eq!(n, size_a + size_b);
    let mut pair = vec![None; n];
    for i in 0..size_a {
        if pair[i].is_some() { continue; }
        dfs(g, &mut pair, &mut vec![false; n], i);
    }
    pair.iter().take(size_a).filter(|v| v.is_some()).count()
}


/// O(E\sqrt{V})
/// references
/// - https://en.wikipedia.org/wiki/Maximum_cardinality_matching
/// - https://misteer.hatenablog.com/entry/hopcroft-karp
/// - https://tjkendev.github.io/procon-library/python/max_flow/hopcroft-karp.html
pub fn hopcroft_karp() {

}
