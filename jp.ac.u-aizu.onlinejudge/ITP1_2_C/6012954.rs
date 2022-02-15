// #[derive(Default)]
// pub struct Scanner<R: std::io::Read> {
//     reader: std::io::StdinLock<'a>,
// }
pub struct Scanner<'a> {
    reader: std::io::StdinLock<'a>,
}

/// ```
/// let mut sc: Scanner = Scanner::default();
/// let a: i32 = sc.scan::<i32>();
/// ```
// impl<R: std::io::Read> Scanner<R> {
impl<'a> Scanner<'a> {
        pub fn new() -> Self {
        let stdin = Box::leak(Box::new(std::io::stdin()));
        Self { reader: stdin.lock() }
    }

    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c|c.unwrap()as char)
        .skip_while(|c|c.is_whitespace())
        .take_while(|c|!c.is_whitespace())
        .collect::<String>().parse::<T>().ok().unwrap()
    }

}


pub fn scan<T: std::str::FromStr>() -> T {
    use std::io::Read;
    std::io::stdin().lock().bytes().map(|c|c.unwrap()as char)
    .skip_while(|c|c.is_whitespace())
    .take_while(|c|!c.is_whitespace())
    .collect::<String>().parse::<T>().ok().unwrap()
}


// #[allow(warnings)]
fn main() {
    use std::io::Write;
    let mut sc = Scanner::new();
    let out = &mut std::io::BufWriter::new(std::io::stdout());
    let n = 3;
    let mut a = vec![0; n];
    for i in 0..n { a[i] = sc.scan(); }
    a.sort();
    writeln!(out, "{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")).unwrap();
}
