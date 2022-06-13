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

    let eids = mst_boruvka(n, &edges);
    let s = eids.iter().map(|&i| edges[i].2).sum::<i64>();
    writeln!(writer, "{}", s)?;

    writer.flush()?;
    Ok(())
}

/// O(E\log{V})
pub fn mst_boruvka(
    v_size: usize,
    undirected_edges: &[(usize, usize, i64)],
) -> Vec<usize> {
    let m = undirected_edges.len();
    let mut is_added = vec![false; m];
    let mut added_eids = vec![];
    loop {
        let labels = connected_components_dfs(
            v_size,
            &added_eids
                .iter()
                .map(|&i| {
                    let e: (usize, usize, i64) = undirected_edges[i];
                    (e.0, e.1)
                })
                .collect::<Vec<_>>(),
        );
        let k = *labels.iter().max().unwrap() + 1;
        if k == 1 {
            break;
        }
        let mut min_edge = vec![m; k];
        for (i, &(u, v, w)) in undirected_edges.iter().enumerate() {
            if is_added[i] {
                continue;
            }
            let u = labels[u];
            let v = labels[v];
            if u == v {
                continue;
            }
            if min_edge[u] == m || w < undirected_edges[min_edge[u]].2 {
                min_edge[u] = i;
            }
            if min_edge[v] == m || w < undirected_edges[min_edge[v]].2 {
                min_edge[v] = i;
            }
        }
        for i in min_edge {
            if i == m || is_added[i] {
                continue;
            }
            added_eids.push(i);
            is_added[i] = true;
        }
    }
    added_eids
}

pub fn connected_components_dfs(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut g = vec![vec![]; v_size];
    for &(u, v) in undirected_edges.iter() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut labels = vec![v_size; v_size];
    let mut label = 0;
    fn dfs(g: &[Vec<usize>], labels: &mut Vec<usize>, label: usize, u: usize) {
        labels[u] = label;
        for &v in g[u].iter() {
            if labels[v] == g.len() {
                dfs(g, labels, label, v);
            }
        }
    }
    for i in 0..v_size {
        if labels[i] != v_size {
            continue;
        }
        dfs(&g, &mut labels, label, i);
        label += 1;
    }
    labels
}

pub fn connected_components_uf(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut uf = UnionFind::new(v_size);
    for &(u, v) in undirected_edges {
        uf.unite(u, v);
    }
    uf.get_labels()
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

impl UnionFind {
    pub fn get_labels(&mut self) -> Vec<usize> {
        let n = self.size();
        let mut labels = vec![n; n];
        let mut label = 0;
        for i in 0..n {
            let root = self.find_root(i);
            if labels[root] == n {
                labels[root] = label;
                label += 1;
            }
            labels[i] = labels[root];
        }
        labels
    }
}
