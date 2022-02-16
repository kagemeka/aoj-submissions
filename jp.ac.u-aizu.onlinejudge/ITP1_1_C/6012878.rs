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
    let x: i32 = scan();
    let y: i32 = scan();
    writeln!(out, "{} {}", x * y, (x + y) * 2);
}

