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

type Mat<T> = Vec<Vec<Option<T>>>;

pub fn floyd_warshall<T>(adj_mat: Mat<T>) -> Mat<T>
where
    T: Copy + std::ops::Add<Output = T> + Ord,
{
    let mut g = adj_mat;
    let n = g.len();
    for k in 0..n {
        for i in 0..n {
            if g[i][k].is_none() {
                continue;
            }
            let d0 = g[i][k].unwrap();
            for j in 0..n {
                if g[k][j].is_none() {
                    continue;
                }
                let d = d0 + g[k][j].unwrap();
                if g[i][j].is_none() || Some(d) < g[i][j] {
                    g[i][j] = Some(d);
                }
            }
        }
    }
    g
}

fn main() {
    let n: usize = read();
    let m: usize = read();

    let mut g = vec![vec![None; n]; n];
    for i in 0..n {
        g[i][i] = Some(0);
    }
    for _ in 0..m {
        let s: usize = read();
        let t: usize = read();
        let d: i64 = read();
        if g[s][t].is_none() || Some(d) < g[s][t] {
            g[s][t] = Some(d);
        }
    }

    g = floyd_warshall(g);
    for i in 0..n {
        if g[i][i] < Some(0) {
            println!("NEGATIVE CYCLE");
            return;
        }
    }
    for i in 0..n {
        for j in 0..n {
            if g[i][j].is_none() {
                print!("INF");
            } else {
                print!("{}", g[i][j].unwrap());
            }
            print!(
                "{}",
                if j == n - 1 { '\n' } else { ' ' }
            );
        }
    }
}

