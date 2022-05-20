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

// TODO: use generic type for priority queues instead of binary heap.
pub fn dijkstra_sparse(
    sparse_graph: &[Vec<(usize, u64)>],
    src: usize,
) -> Vec<Option<u64>> {
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
        for &(v, weight) in sparse_graph[u].iter() {
            let dv = du + weight;
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

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = reader.read::<usize>()?;
        let v = reader.read::<usize>()?;
        let w = reader.read::<u64>()?;
        graph[u].push((v, w));
    }
    let res = dijkstra_sparse(&graph, r);
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

