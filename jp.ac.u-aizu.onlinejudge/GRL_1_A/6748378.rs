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

pub fn dijkstra<T>(adj_list: Vec<Vec<(usize, T)>>, s: usize) -> Vec<Option<T>>
where
    T: Copy + Ord + std::ops::Add<Output = T> + Default,
{
    use std::{cmp::Reverse, collections::BinaryHeap};
    let g = adj_list;
    let n = g.len();
    let mut dist: Vec<Option<T>> = vec![None; n];
    dist[s] = Some(T::default());
    let mut q = BinaryHeap::new();
    q.push(Reverse((dist[s].unwrap(), s)));
    while let Some(Reverse((du, u))) = q.pop() {
        if Some(du) > dist[u] {
            continue;
        }
        for &(v, w) in &g[u] {
            let dv = du + w;
            if dist[v].is_some() && Some(dv) >= dist[v] {
                continue;
            }
            dist[v] = Some(dv);
            q.push(Reverse((dv, v)));
        }
    }
    dist
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let src: usize = read();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let s: usize = read();
        let t: usize = read();
        let d: u64 = read();
        g[s].push((t, d));
    }
    let dist = dijkstra(g, src);
    for &d in dist.iter() {
        if let Some(d) = d {
            println!("{}", d);
        } else {
            println!("INF");
        }
    }
}

