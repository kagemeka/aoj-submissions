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

#[derive(Debug, PartialEq)]
pub struct NegativeCycleError {
    msg: &'static str,
}

impl NegativeCycleError {
    pub fn new() -> Self { Self { msg: "Negative Cycle Found." } }
}

impl std::fmt::Display for NegativeCycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NegativeCycleError {
    fn description(&self) -> &str { &self.msg }
}

pub trait Edge {}

pub trait UndirectedEdge {
    type V;
    fn u(&self) -> &Self::V;
    fn v(&self) -> &Self::V;
}

pub trait ToDirected {
    type E;
    fn to_directed(self) -> Self::E;
}

pub trait From {
    type V;
    fn from(&self) -> &Self::V;
}

pub trait To {
    type V;

    fn to(&self) -> &Self::V;
}

pub trait DirectedEdge: To {}

pub trait Reversed {
    fn reversed(self) -> Self;
}

pub trait Value {
    type T;
    fn value(&self) -> &Self::T;
}

pub fn edges_to_directed<E>(undirected_edges: Vec<E>) -> Vec<E>
where
    E: Clone + Reversed + ToDirected<E = E>,
{
    let di_edges = undirected_edges.into_iter().map(|e| e.to_directed());
    let rev = di_edges.clone().map(|e| e.reversed());
    di_edges.chain(rev).collect()
}

#[derive(Debug)]
pub struct AdjacencyList<E> {
    pub(crate) edges: Vec<Vec<E>>,
}

impl<E> AdjacencyList<E> {
    pub fn size(&self) -> usize { self.edges.len() }

    pub fn edges(&self) -> &[Vec<E>] { &self.edges }

    pub fn new(size: usize) -> Self {
        Self {
            edges: (0..size).map(|_| Vec::new()).collect(),
        }
    }

    pub fn add_node(&mut self) { self.edges.push(Vec::new()); }
}

impl<E> std::ops::Index<usize> for AdjacencyList<E> {
    type Output = Vec<E>;

    fn index(&self, i: usize) -> &Self::Output { &self.edges[i] }
}

impl<T> std::ops::IndexMut<usize> for AdjacencyList<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.edges[i]
    }
}

impl<E> std::convert::From<(usize, Vec<E>)> for AdjacencyList<E>
where
    E: From<V = usize>,
{
    fn from((size, edges): (usize, Vec<E>)) -> Self {
        let mut g = Self::new(size);
        for e in edges.into_iter() {
            g[*e.from()].push(e);
        }
        g
    }
}

impl<E> std::convert::Into<Vec<E>> for AdjacencyList<E>
where
    E: From<V = usize>,
{
    fn into(self) -> Vec<E> {
        self.edges
            .into_iter()
            .enumerate()
            .flat_map(|(u, edges)| {
                assert!(edges.iter().all(|e| e.from() == &u));
                edges
            })
            .collect()
    }
}

impl<T> ToDirected for (usize, usize, T) {
    type E = Self;

    fn to_directed(self) -> Self::E { self }
}

impl<T> Reversed for (usize, usize, T) {
    fn reversed(mut self) -> Self {
        std::mem::swap(&mut self.0, &mut self.1);
        self
    }
}

impl<T> From for (usize, usize, T) {
    type V = usize;

    fn from(&self) -> &Self::V { &self.0 }
}

impl<T> To for (usize, usize, T) {
    type V = usize;

    fn to(&self) -> &Self::V { &self.1 }
}

impl<T> Value for (usize, usize, T) {
    type T = T;

    fn value(&self) -> &Self::T { &self.2 }
}

pub fn dijkstra_sparse<E>(
    sparse_graph: &[Vec<E>],
    src: usize,
) -> Vec<Option<u64>>
where
    E: To<V = usize> + Value<T = u64>,
{
    use std::cmp::Reverse;
    let n = sparse_graph.len();
    let mut dist = vec![None; n];
    let mut hq = std::collections::BinaryHeap::<Reverse<(u64, _)>>::new();
    dist[src] = Some(0);
    hq.push(Reverse((0, src)));
    while let Some(Reverse((du, u))) = hq.pop() {
        if du > dist[u].unwrap() {
            continue;
        }
        for e in sparse_graph[u].iter() {
            let v = *e.to();
            let dv = du + e.value();
            if dist[v].is_some() && dv >= dist[v].unwrap() {
                continue;
            }
            dist[v] = Some(dv);
            hq.push(Reverse((dv, v)));
        }
    }
    dist
}

// #[allow(warnings)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut reader = locked_stdin_reader();
    let mut writer = locked_stdin_buf_writer();

    let n = reader.read::<usize>()?;
    let m = reader.read::<usize>()?;
    let r = reader.read::<usize>()?;
    let edges = (0..m)
        .map(|_| {
            (
                reader.read::<usize>().unwrap(),
                reader.read::<usize>().unwrap(),
                reader.read::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let g = AdjacencyList::from((n, edges));

    let res = dijkstra_sparse(g.edges(), r);
    for d in res.iter() {
        if let Some(d) = d {
            writeln!(writer, "{}", d)?;
        } else {
            writeln!(writer, "INF")?;
        }
    }
    writer.flush()?;
    Ok(())
}
