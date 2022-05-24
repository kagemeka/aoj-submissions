pub struct ReadWrapper<R> {
    reader: R,
    tokens: Vec<String>,
}

impl<R> ReadWrapper<R> {
    pub fn new(reader: R) -> Self { Self { reader, tokens: vec![] } }
}

impl<R: std::io::BufRead> ReadWrapper<R> {
    pub fn read<T: std::str::FromStr>(
        &mut self,
    ) -> Result<T, <T as std::str::FromStr>::Err> {
        while self.tokens.is_empty() {
            let mut buf = String::new();
            self.reader.read_line(&mut buf).unwrap();
            self.tokens =
                buf.split_whitespace().map(str::to_string).rev().collect();
        }
        self.tokens.pop().unwrap().parse::<T>()
    }
}

pub fn locked_stdin_reader() -> ReadWrapper<std::io::StdinLock<'static>> {
    let stdin = Box::leak(Box::new(std::io::stdin()));
    ReadWrapper::new(stdin.lock())
}

pub fn locked_stdin_buf_writer()
-> std::io::BufWriter<std::io::StdoutLock<'static>> {
    let stdout = Box::leak(Box::new(std::io::stdout()));
    std::io::BufWriter::new(stdout.lock())
}

// #[allow(warnings)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut reader = locked_stdin_reader();
    let mut writer = locked_stdin_buf_writer();

    let n: usize = reader.read()?;
    let m: usize = reader.read()?;

    let edges = (0..m)
        .map(|_| {
            let a: usize = reader.read::<usize>().unwrap();
            let b: usize = reader.read().unwrap();
            let w: i64 = reader.read().unwrap();
            (a, b, w)
        })
        .collect::<Vec<_>>();

    let mut g = vec![vec![None; n]; n];
    for (a, b, w) in edges {
        g[a][b] = Some(w);
        g[b][a] = Some(w);
    }
    let edges = mst_prim_dense(&g);
    let s = edges.iter().map(|&(u, v)| g[u][v].unwrap()).sum::<i64>();
    writeln!(writer, "{}", s)?;

    writer.flush()?;
    Ok(())
}

pub fn is_adjacency_matrix<T>(graph: &[Vec<T>]) -> bool {
    let n = graph.len();
    graph.iter().all(|row| row.len() == n)
}

pub fn is_undirected_dense_graph<T: Ord>(graph: &[Vec<T>]) -> bool {
    if !is_adjacency_matrix(graph) {
        return false;
    }
    let n = graph.len();
    (0..n - 1).all(|i| (i + 1..n).all(|j| graph[i][j] == graph[j][i]))
}

// return edge list (u, v)
/// O(V^2)
pub fn mst_prim_dense(
    undirected_dense_graph: &[Vec<Option<i64>>],
) -> Vec<(usize, usize)> {
    let g = undirected_dense_graph;
    let n = g.len();
    // for u in 0..n - 1 {
    //     for v in u + 1..n {
    //         assert_eq!(g[u][v], g[v][u]);
    //     }
    // }
    assert!(is_undirected_dense_graph(g));
    let mut prev: Vec<Option<usize>> = vec![None; n];
    let mut visited = vec![false; n];
    let mut added = Vec::with_capacity(n - 1);
    for _ in 0..n {
        let (pre, u, _) = (0..n)
            .filter_map(|i| {
                if visited[i] || prev[i].is_none() {
                    return None;
                }
                let p = prev[i].unwrap();
                Some((
                    p,
                    i,
                    g[prev[i].unwrap()][i].unwrap(),
                ))
            })
            .min_by_key(|&(.., w)| w)
            .unwrap_or_default();
        visited[u] = true;
        if pre != u {
            if pre > u {
                let mut pre = pre;
                let mut u = u;
                std::mem::swap(&mut pre, &mut u);
            }
            added.push((pre, u));
        }
        for v in 0..n {
            if visited[v]
                || g[u][v].is_none()
                || prev[v].is_some() && g[u][v] >= g[prev[v].unwrap()][v]
            {
                continue;
            }
            prev[v] = Some(u);
        }
    }
    debug_assert_eq!(added.len(), n - 1);
    added
}

#[derive(Debug)]
pub struct UnionFind {
    data: Vec<isize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self { Self { data: vec![-1; size] } }

    pub fn size(&self) -> usize { self.data.len() }

    pub fn find_root(&mut self, u: usize) -> usize {
        if self.data[u] < 0 {
            return u;
        }
        self.data[u] = self.find_root(self.data[u] as usize) as isize;
        self.data[u] as usize
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let mut u = self.find_root(u);
        let mut v = self.find_root(v);
        if u == v {
            return;
        }
        if self.data[u] > self.data[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.data[u] += self.data[v];
        self.data[v] = u as isize;
    }

    pub fn size_of(&mut self, u: usize) -> usize {
        let u = self.find_root(u);
        -self.data[u] as usize
    }
}

impl UnionFind {
    pub fn are_same(&mut self, u: usize, v: usize) -> bool {
        self.find_root(u) == self.find_root(v)
    }
}

