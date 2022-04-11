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
    let f = prime_factorize(n);
    write!(out, "{}: ", n).unwrap();
    for (p, c) in f.iter() {
        for _ in 0..*c {
            write!(out, "{} ", p).unwrap();
        }
    }
    writeln!(out, "").unwrap();
}




pub fn least_prime_factor(n: usize) -> Vec<usize> {
    assert!(n >= 2);
    let mut s: Vec<usize> = (0..n).collect();
    s[1] = 0;
    let mut i = 0;
    while i * i < n - 1 {
        i += 1;
        if s[i as usize] != i { continue; }
        for j in (i * i..n).step_by(i as usize) {
            if s[j as usize] == j { s[j as usize] = i; }
        }
    }
    s
}

pub fn greatest_prime_factor(n: usize) -> Vec<usize> {
    assert!(n >= 2);
    let mut s: Vec<usize> = (0..n).collect();
    s[1] = 0;
    let mut i = 0;
    while i < n - 1 {
        i += 1;
        if s[i as usize] != i { continue; }
        for j in (i * 2..n).step_by(i as usize) {
            s[j as usize] = i;
        }
    }
    s
}


pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let lpf = least_prime_factor(n);
    (0..n).map(|i| i >= 2 && i == lpf[i as usize]).collect()
}


pub fn find_prime_numbers(n: usize) -> Vec<usize> {
    let is_prime = sieve_of_eratosthenes(n);
    (0..n).filter(|i| is_prime[*i as usize]).collect()
}


pub fn prime_factorize(mut n: usize) -> std::collections::BTreeMap<usize, usize> {
    let mut cnt = std::collections::BTreeMap::new();
    let mut i = 1;
    while i * i < n {
        i += 1;
        if n % i != 0 { continue; }
        while n % i == 0 {
            n /= i;
            *cnt.entry(i).or_insert(0usize) += 1;
        }
    }
    if n > 1 { cnt.insert(n, 1); }
    cnt
}


pub struct PrimeFactorizeLPF {
    lpf: Vec<usize>,
}


impl PrimeFactorizeLPF {
    pub fn new(n: usize) -> Self {
        PrimeFactorizeLPF { lpf: least_prime_factor(n) }
    }

    pub fn factorize(&self, mut n: usize) -> std::collections::BTreeMap<usize, usize> {
        let mut cnt = std::collections::BTreeMap::new();
        while n > 1 {
            let p = self.lpf[n] as usize;
            n /= p;
            *cnt.entry(p).or_insert(0usize) += 1;
        }
        cnt
    }
}


pub fn count_prime_factors(n: usize) -> Vec<usize> {
    let mut cnt = vec![0; n as usize];
    for p in find_prime_numbers(n).into_iter().map(|x| x as usize) {
        for i in (p..n).step_by(p) { cnt[i] += 1; }
    }
    cnt
}
