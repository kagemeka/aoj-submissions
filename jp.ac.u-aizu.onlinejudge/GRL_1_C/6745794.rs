pub mod io {

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

    pub fn locked_stdout_buf_writer()
    -> std::io::BufWriter<std::io::StdoutLock<'static>> {
        let stdout = Box::leak(Box::new(std::io::stdout()));
        std::io::BufWriter::new(stdout.lock())
    }

    /// reference
    /// https://users.rust-lang.org/t/show-value-only-in-debug-mode/43686/3
    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! debug {
    ($($x:tt)*) => {
        {
            // default in debug mode
            #[cfg(debug_assertions)]
            {
                std::dbg!($($x)*)
            }

            // default in release mode
            #[cfg(not(debug_assertions))]
            {
                ($($x)*)
            }
        }
    }
}

    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! write_vec {
    ($writer:ident, $values:expr) => {
        write_vec!($writer, $values, sep: ' ');
    };

    ($writer:ident, $values:expr,sep: $sep:expr) => {
        let n = $values.len();
        if n == 0 {
            writeln!($writer).unwrap();
        } else {
            for i in 0..n - 1 {
                write!(
                    $writer,
                    "{}{}",
                    $values[i], $sep
                )
                .unwrap();
            }
            writeln!($writer, "{}", $values[n - 1]).unwrap();
        }
    };
}

    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! write_all {
    ($writer:ident) => {
        writeln!($writer).unwrap();
    };

    ($writer:ident, $v:expr) => {
        writeln!($writer, "{}", $v).unwrap();
    };

    ($writer:ident, $v:expr, $($values:expr),+) => {
        write!($writer, "{} ", $v).unwrap();
        write_all!($writer, $($values),*);
    };
}

    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! read_vec {
        ($reader:ident, $type:ty, $n:expr) => {
            (0..$n)
                .map(|_| $reader.read::<$type>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        };
    }
}

type Mat<T> = Vec<Vec<T>>;

pub fn floyd_warshall<T>(mut g: Mat<T>) -> Mat<T>
where
    T: Ord + std::ops::Add<Output = T> + Copy,
{
    let n = g.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }
    g
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct DefaultData(pub i64);

impl DefaultData {
    const INF: i64 = 1 << 60;
}

impl std::ops::Add<Self> for DefaultData {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let inf = Self::INF;
        if self.0 == inf || rhs.0 == inf {
            Self(inf)
        } else {
            Self(self.0 + rhs.0)
        }
    }
}

// TODO: main
// #[allow(warnings)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    use io::*;
    let mut reader = locked_stdin_reader();
    let mut writer = locked_stdout_buf_writer();

    type D = DefaultData;
    let n: usize = reader.read()?;
    let m: usize = reader.read()?;

    let mut g = vec![vec![DefaultData(D::INF); n]; n];
    for i in 0..n {
        g[i][i] = DefaultData(0);
    }
    for _ in 0..m {
        let s: usize = reader.read()?;
        let t: usize = reader.read()?;
        let d: i64 = reader.read()?;
        g[s][t] = g[s][t].min(DefaultData(d));
    }

    let g = floyd_warshall(g);

    for i in 0..n {
        if g[i][i].0 < 0 {
            writeln!(writer, "NEGATIVE CYCLE")?;
            return Ok(());
        }
    }

    for i in 0..n {
        for j in 0..n {
            if g[i][j].0 == D::INF {
                write!(writer, "INF")?;
            } else {
                write!(writer, "{}", g[i][j].0)?;
            }
            write!(
                writer,
                "{}",
                if j == n - 1 { '\n' } else { ' ' }
            )?;
        }
    }

    writer.flush()?;
    Ok(())
}

