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
    let inf = std::usize::MAX;
    let mut cnt = vec![inf; n + 1];
    cnt[0] = 0;
    let mut dp = std::collections::HashSet::new();
    dp.insert(0usize);
    for c in 0..n {
        let mut ndp = std::collections::HashSet::new();
        for &x in dp.iter() {
            for &i in d.iter() {
                if x + i > n || cnt[x + i] < c + 1 { continue; }
                cnt[x + i] = c + 1;
                ndp.insert(x + i);
            }
        }
        dp = ndp;
        if !dp.contains(&n) { continue; }
        writeln!(out, "{}", c + 1).unwrap();
        break;
    }
    
}

