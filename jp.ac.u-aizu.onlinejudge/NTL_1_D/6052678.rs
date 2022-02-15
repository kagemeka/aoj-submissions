pub struct Scanner<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    /// let stdin = std::io::stdin();
    /// let mut sc = Scanner::new(stdin.lock());
    pub fn new(reader: R) -> Self { Self { reader: reader } }

    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
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
    writeln!(out, "{}", euler_totient(n)).unwrap();
}



/// O(\sqrt{N})
pub fn euler_totient(mut n: usize) -> usize {
    let mut cnt = n;
    let mut i = 1;
    while i * i < n {
        i += 1;
        if n % i != 0 { continue; }
        cnt = cnt / i * (i - 1);
        while n % i == 0 { n /= i; }
    }
    if n > 1 { cnt = cnt / n * (n - 1); }
    cnt
}


pub struct EulerTotientLPF {}
