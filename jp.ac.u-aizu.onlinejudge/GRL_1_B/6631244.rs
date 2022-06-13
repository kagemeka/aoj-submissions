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

#[derive(Debug)]
pub struct NegativeCycleError {
    msg: &'static str,
}

impl NegativeCycleError {
    pub(crate) fn new() -> Self { Self { msg: "Negative Cycle Found." } }
}

impl std::fmt::Display for NegativeCycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NegativeCycleError {
    fn description(&self) -> &str { &self.msg }
}

pub fn bellman_ford_sparse(
    nodes_size: usize,
    directed_edges: &[(usize, usize, i64)],
    src: usize,
) -> Result<Vec<Option<i64>>, NegativeCycleError> {
    let mut dist = vec![None; nodes_size];
    dist[src] = Some(0);
    for i in 0..nodes_size {
        for &(u, v, w) in directed_edges {
            if dist[u].is_none()
                || dist[v].is_some() && dist[u].unwrap() + w >= dist[v].unwrap()
            {
                continue;
            }
            if i == nodes_size - 1 {
                return Err(NegativeCycleError::new());
            }
            dist[v] = Some(dist[u].unwrap() + w);
        }
    }
    Ok(dist)
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
                reader.read::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let res = bellman_ford_sparse(n, &edges, r);
    if res.is_err() {
        writeln!(writer, "NEGATIVE CYCLE")?;
        return Ok(());
    }
    let res = res?;
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
