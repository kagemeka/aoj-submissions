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
    let m: usize = sc.scan();
    let mut d = vec![0usize; m];
    for i in 0..m {
        d[i] = sc.scan();
    }
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for cnt in 0..n {
        for i in (1..n + 1).rev() {
            for &j in d.iter() {
                if j > i { continue; }
                dp[i] |= dp[i - j];
            }
        }
        if !dp[n] { continue; }
        writeln!(out, "{}", cnt + 1).unwrap();
        break;
    }
    
}

