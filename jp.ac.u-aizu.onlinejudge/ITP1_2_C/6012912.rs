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
        let out = &mut std::io::BufWriter::new(std::io::stdout());
        let n = 3;
        let mut a = vec![0; n];
        for i in 0..n { a[i] = scan(); }
        a.sort();
        writeln!(out, "{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")).unwrap();   
    }

