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

pub fn floyd_warshall(
    weight_matrix: Vec<Vec<Option<i64>>>,
) -> Result<Vec<Vec<Option<i64>>>, NegativeCycleError> {
    let n = weight_matrix.len();
    let mut g = weight_matrix;
    assert!((0..n).all(|i| g[i].len() == n));
    for i in 0..n {
        if g[i][i].is_none() || g[i][i] > Some(0) {
            g[i][i] = Some(0)
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if g[i][k].is_none() || g[k][j].is_none() {
                    continue;
                }
                let d = Some(g[i][k].unwrap() + g[k][j].unwrap());
                if g[i][j].is_none() || d < g[i][j] {
                    g[i][j] = d;
                }
            }
        }
    }
    if (0..n).any(|i| g[i][i] < Some(0)) {
        Err(NegativeCycleError::new())
    } else {
        Ok(g)
    }
}

// #[allow(warnings)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut reader = locked_stdin_reader();
    let mut writer = locked_stdin_buf_writer();

    let n = reader.read::<usize>()?;
    let m = reader.read::<usize>()?;

    let mut graph = vec![vec![None; n]; n];
    for _ in 0..m {
        let u = reader.read::<usize>()?;
        let v = reader.read::<usize>()?;
        let w = reader.read::<i64>()?;
        graph[u][v] = Some(w);
    }

    let res = floyd_warshall(graph);
    if res.is_err() {
        writeln!(writer, "NEGATIVE CYCLE")?;
        return Ok(());
    }
    for row in res.unwrap() {
        for (i, &w) in row.iter().enumerate() {
            let ans = if w.is_none() {
                "INF".to_string()
            } else {
                w.unwrap().to_string()
            };

            write!(
                writer,
                "{}{}",
                ans,
                if i == n - 1 { '\n' } else { ' ' }
            )?;
        }
    }

    writer.flush()?;
    Ok(())
}

