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
        let m: i32 = sc.scan();
        let f: i32 = sc.scan();
        let r: i32 = sc.scan();
        if m == -1 && f == -1 && r == -1 { break; }

        let ans: char;
        if m == -1 || f == -1 || m + f < 30 {
            ans = 'F';
        } else if m + f >= 80 {
            ans = 'A';
        } else if m + f >= 65 {
            ans = 'B';
        } else if m + f >= 50 || r >= 50 {
            ans = 'C';
        } else {
            ans = 'D';
        }
        writeln!(out, "{}", ans).unwrap();
    }

}
