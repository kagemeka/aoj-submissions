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

pub fn desopo_pape<T>(
    adj_list: Vec<Vec<(usize, T)>>,
    src: usize,
) -> Vec<Option<T>>
where
    T: Ord + std::ops::Add<Output = T> + Copy + Default,
{
    use std::collections::VecDeque;
    let g = adj_list;
    let n = g.len();
    let mut dist = vec![None; n];
    dist[src] = Some(T::default());
    let mut q = VecDeque::new();
    q.push_back(src);

    #[derive(PartialEq, Clone, Copy)]
    enum State {
        OnStack,
        Popped,
        None,
    }

    let mut state = vec![State::None; n];
    while let Some(u) = q.pop_front() {
        state[u] = State::Popped;
        debug_assert!(dist[u].is_some());
        for &(v, w) in g[u].iter() {
            let dv = dist[u].unwrap() + w;
            if dist[v].is_some() && Some(dv) >= dist[v] {
                continue;
            }
            dist[v] = Some(dv);
            match state[v] {
                State::OnStack => continue,
                State::Popped => {
                    q.push_back(v);
                    state[v] = State::OnStack
                },
                State::None => {
                    q.push_front(v);
                    state[v] = State::OnStack;
                },
            }
        }
    }
    dist
}

fn main() {
    use std::io::Write;
    let out = std::io::stdout();
    let writer = &mut std::io::BufWriter::new(out.lock());
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
    let dist = desopo_pape(g, src);
    for &d in dist.iter() {
        if let Some(d) = d {
            writeln!(writer, "{}", d).unwrap();
        } else {
            writeln!(writer, "INF").unwrap();
        }
    }
}

