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

    let n: usize = reader.read::<usize>()?;
    let m: usize = reader.read::<usize>()?;

    let edges = (0..m)
        .map(|_| {
            let a: usize = reader.read::<usize>().unwrap();
            let b: usize = reader.read::<usize>().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let mut g = vec![vec![]; n];
    for &(a, b) in &edges {
        g[a].push(b);
    }
    // let labels = scc_tarjan_lowlink(&g);
    let labels = scc_path_based(&g);

    let q: usize = reader.read()?;
    for _ in 0..q {
        let u: usize = reader.read()?;
        let v: usize = reader.read()?;
        writeln!(
            writer,
            "{}",
            if labels[u] == labels[v] { 1 } else { 0 }
        )
        .unwrap();
    }

    writer.flush()?;
    Ok(())
}

/// essentially same with Tarjan's Lowlink algorithm
pub fn scc_path_based(sparse_graph: &[Vec<usize>]) -> Vec<usize> {
    SCCPathBased::compute_labels(sparse_graph)
}

struct SCCPathBased<'a> {
    graph: &'a [Vec<usize>],
    stack: Vec<usize>,
    low_stack: Vec<usize>,
    orders: Vec<usize>,
    order: usize,
    labels: Vec<usize>,
    label: usize,
}

impl<'a> SCCPathBased<'a> {
    pub fn compute_labels(
        directed_sparse_graph: &'a [Vec<usize>],
    ) -> Vec<usize> {
        let mut scc = Self::new(directed_sparse_graph);
        for i in 0..scc.size() {
            if scc.labels[i] == scc.size() {
                Self::labeling(&mut scc, i);
            }
        }
        Self::topological_sort(scc.labels)
    }

    fn new(graph: &'a [Vec<usize>]) -> Self {
        let n = graph.len();
        Self {
            graph,
            stack: vec![],
            low_stack: vec![],
            orders: vec![n; n],
            order: 0,
            labels: vec![n; n],
            label: 0,
        }
    }

    fn size(&self) -> usize { self.graph.len() }

    fn labeling(scc: &mut Self, u: usize) {
        scc.orders[u] = scc.order;
        scc.order += 1;
        scc.stack.push(u);
        scc.low_stack.push(u);
        for &v in &scc.graph[u] {
            if scc.orders[v] == scc.size() {
                Self::labeling(scc, v);
            } else if scc.labels[v] == scc.size() {
                while scc.orders[*scc.low_stack.last().unwrap()] > scc.orders[v]
                {
                    scc.low_stack.pop();
                }
            }
        }
        if scc.low_stack.last().unwrap() != &u {
            return;
        }
        loop {
            let v = scc.stack.pop().unwrap();
            scc.labels[v] = scc.label;
            if v == u {
                break;
            }
        }
        scc.label += 1;
        scc.low_stack.pop();
    }

    fn topological_sort(labels: Vec<usize>) -> Vec<usize> {
        let k = *labels.iter().max().unwrap();
        labels.into_iter().map(|l| k - l).collect::<Vec<_>>()
    }
}

/// essentially same with Path-Based algorithm
pub fn scc_tarjan_lowlink(sparse_graph: &[Vec<usize>]) -> Vec<usize> {
    SCCTarjanLowLink::compute_labels(sparse_graph)
}

// just a data wrapper to avoid having many parameters in a dfs function.
struct SCCTarjanLowLink<'a> {
    graph: &'a [Vec<usize>],
    stack: Vec<usize>,
    orders: Vec<usize>,
    order: usize,
    low_order: Vec<usize>,
    labels: Vec<usize>,
    label: usize,
}

impl<'a> SCCTarjanLowLink<'a> {
    pub fn compute_labels(
        directed_sparse_graph: &'a [Vec<usize>],
    ) -> Vec<usize> {
        let mut scc = Self::new(directed_sparse_graph);
        for i in 0..scc.size() {
            if scc.labels[i] == scc.size() {
                Self::labeling(&mut scc, i);
            }
        }
        Self::topological_sort(scc.labels)
    }

    fn new(graph: &'a [Vec<usize>]) -> Self {
        let n = graph.len();
        Self {
            graph,
            stack: vec![],
            orders: vec![n; n],
            order: 0,
            low_order: vec![n; n],
            labels: vec![n; n],
            label: 0,
        }
    }

    fn size(&self) -> usize { self.graph.len() }

    // labels are fixed in topologically reverse order of components.
    fn labeling(scc: &mut Self, u: usize) {
        scc.orders[u] = scc.order;
        scc.order += 1;
        scc.stack.push(u);
        for &v in &scc.graph[u] {
            if scc.orders[v] == scc.size() {
                Self::labeling(scc, v);
                scc.low_order[u] = std::cmp::min(
                    scc.low_order[u],
                    scc.low_order[v],
                );
            } else if scc.labels[v] == scc.size() {
                // v is not in a scc yet.
                scc.low_order[u] =
                    std::cmp::min(scc.low_order[u], scc.orders[v]);
                // `s.t. if low[v] < low[u] then low[u] = low[v]`?
                // but low[v] is still under computing.
                // here,
                // it's just enough to know whether
                // low[u] can be smaller than or equal to ord[v].
            }
        }
        if scc.low_order[u] < scc.orders[u] {
            return;
        }
        // a scc is fixed.
        loop {
            let v = scc.stack.pop().unwrap();
            scc.labels[v] = scc.label;
            if v == u {
                break;
            }
        }
        scc.label += 1;
    }

    fn topological_sort(labels: Vec<usize>) -> Vec<usize> {
        // after labeling, labels are still reverse order
        // in a point of topologicality.
        let k = *labels.iter().max().unwrap();
        labels.into_iter().map(|l| k - l).collect::<Vec<_>>()
    }
}

