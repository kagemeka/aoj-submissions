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

    loop {
        let h: usize = sc.scan();
        let w: usize = sc.scan();
        if h == 0 && w == 0 { break; }
        writeln!(out, "{}", "#".repeat(w)).unwrap();
        let mut s = String::from("#");
        s = s +  &".".repeat(w - 2) + "#";
        for _ in 0..h - 2 {
            writeln!(out, "{}", s).unwrap();
        }
        writeln!(out, "{}", "#".repeat(w)).unwrap();
        writeln!(out, "").unwrap();
    }

}
