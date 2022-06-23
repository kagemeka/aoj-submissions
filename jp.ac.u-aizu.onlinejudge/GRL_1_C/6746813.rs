pub fn read<T: std::str::FromStr>() -> T {
    use std::io::Read;
    std::io::stdin()
        .lock()
        .by_ref()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<T>()
        .ok()
        .unwrap()
}

type G = Vec<Vec<i64>>;
const INF: i64 = std::i64::MAX;

pub fn floyd_warshall(mut g: G) -> G {
    let n = g.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if g[i][k] != INF && g[k][j] != INF {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }
    }
    g
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut g = vec![vec![INF; n]; n];
    for i in 0..n {
        g[i][i] = 0;
    }
    for _ in 0..m {
        let s: usize = read();
        let t: usize = read();
        let d: i64 = read();
        g[s][t] = g[s][t].min(d);
    }
    g = floyd_warshall(g);
    for i in 0..n {
        if g[i][i] == INF {
            println!("NEGATIVE CYCLE");
            return;
        }
    }
    for i in 0..n {
        for j in 0..n {
            if g[i][j] == INF {
                print!("INF");
            } else {
                print!("{}", g[i][j]);
            }
            print!(
                "{}",
                if j == n - 1 { '\n' } else { ' ' }
            );
        }
    }
}

