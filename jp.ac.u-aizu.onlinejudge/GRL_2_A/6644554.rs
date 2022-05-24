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

    let eids = mst_prim_sparse(n, &edges);
    let s = eids.iter().map(|&i| edges[i].2).sum::<i64>();
    writeln!(writer, "{}", s)?;

    writer.flush()?;
    Ok(())
}

pub fn mst_prim_sparse(
    v_size: usize,
    undirected_edges: &[(usize, usize, i64)],
) -> Vec<usize> {
    use std::cmp::Reverse;
    let mut g = vec![vec![]; v_size];
    for (i, &(u, v, w)) in undirected_edges.iter().enumerate() {
        g[u].push((v, w, i));
        g[v].push((u, w, i));
    }

    let mut hq = std::collections::BinaryHeap::new();
    hq.push((Reverse(0), 0)); // weight, eid
    let mut min_weight_to = vec![None; v_size];
    min_weight_to[0] = Some(0);
    let mut edge_to = vec![None; v_size];
    let mut visited = vec![false; v_size];
    while let Some((_, u)) = hq.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        for &(v, w, i) in &g[u] {
            if visited[v]
                || min_weight_to[v].is_some() && w >= min_weight_to[v].unwrap()
            {
                continue;
            }
            min_weight_to[v] = Some(w);
            hq.push((Reverse(w), v));
            edge_to[v] = Some(i);
        }
    }
    edge_to.into_iter().filter_map(|e| e).collect()
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

