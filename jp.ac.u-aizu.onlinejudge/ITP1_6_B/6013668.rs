pub struct Scanner<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    /// let stdin = std::io::stdin();
    /// let mut sc = Scanner::new(stdin.lock());
    pub fn new(reader: R) -> Self { Self { reader: reader } }

    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c|c.unwrap()as char)
        .skip_while(|c|c.is_whitespace())
        .take_while(|c|!c.is_whitespace())
        .collect::<String>().parse::<T>().ok().unwrap()
    }
}

// #[allow(warnings)]
fn main() {
    use std::io::Write;
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(std::io::BufReader::new(stdin.lock()));
    let stdout = std::io::stdout();
    let out = &mut std::io::BufWriter::new(stdout.lock());


    let n: usize = sc.scan();
    let a = vec!['S', 'H', 'C', 'D'];
    let mut b = vec![vec![true; 13]; 4];
    for _ in 0..n {
        let c: char = sc.scan();
        let x: usize = sc.scan();
        b[a.iter().position(|x| x == &c).unwrap()][x - 1] = false;
    }
    for i in 0..4usize {
        for j in 0..13usize {
            if !b[i][j] { continue; }
            writeln!(out, "{} {}", a[i], j + 1).unwrap();
        }
    }
}
