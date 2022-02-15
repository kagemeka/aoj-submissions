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


    let mut a = vec![vec![vec![0; 10]; 3]; 4];
    let n: usize = sc.scan();
    for _ in 0..n {
        let b: usize = sc.scan();
        let f: usize = sc.scan();
        let r: usize = sc.scan();
        let v: i32 = sc.scan();
        a[b - 1][f - 1][r - 1] += v;
    }
    for i in 0..4 {
        for j in 0..3 {
            writeln!(out, "{}", String::from(" ") + &a[i][j].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")).unwrap();
        }
        if i < 3 {
            writeln!(out, "{}", "#".repeat(20)).unwrap();
        }
    }

}
