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
    mut weight_matrix: Vec<Vec<i64>>,
) -> Result<Vec<Vec<i64>>, NegativeCycleError> {
    let n = weight_matrix.len();
    assert!((0..n).all(|i| weight_matrix[i].len() == n));
    (0..n).for_each(|i| {
        weight_matrix[i][i] = std::cmp::min(weight_matrix[i][i], 0)
    });
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                weight_matrix[i][j] = std::cmp::min(
                    weight_matrix[i][j],
                    weight_matrix[i][k] + weight_matrix[k][j],
                );
            }
        }
    }
    if (0..n).any(|i| weight_matrix[i][i] < 0) {
        Err(NegativeCycleError::new())
    } else {
        Ok(weight_matrix)
    }
}

// #[allow(warnings)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut reader = locked_stdin_reader();
    let mut writer = locked_stdin_buf_writer();

    let n = reader.read::<usize>()?;
    let m = reader.read::<usize>()?;

    const INF: i64 = 1 << 60;

    let mut graph = vec![vec![INF; n]; n];
    for _ in 0..m {
        let u = reader.read::<usize>()?;
        let v = reader.read::<usize>()?;
        let w = reader.read::<i64>()?;
        graph[u][v] = w;
    }

    let res = floyd_warshall(graph);
    if res.is_err() {
        writeln!(writer, "NEGATIVE CYCLE")?;
        return Ok(());
    }
    for row in res.unwrap() {
        for (i, &w) in row.iter().enumerate() {
            let ans = if w == INF { "INF".to_string() } else { w.to_string() };

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
