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

pub mod container {
    pub trait IsEmpty {
        fn is_empty(&mut self) -> bool;
    }
}

pub mod priority_queue_trait {
    pub trait Push {
        type T;

        fn push(&mut self, x: Self::T);
    }

    pub trait Pop {
        type T;

        fn pop(&mut self) -> Self::T;
    }

    pub trait Top {
        type T;

        fn top(&mut self) -> &Self::T;
    }
}

pub mod graph_edge_trait {
    pub trait Undirected {
        type V;
        fn u(&self) -> &Self::V;
        fn v(&self) -> &Self::V;
    }

    pub trait ToDirected {
        type E;
        fn to_directed(self) -> Self::E;
    }

    pub trait From {
        type V;
        fn from(&self) -> &Self::V;
    }

    pub trait To {
        type V;

        fn to(&self) -> &Self::V;
    }

    pub trait Reverse {
        fn reverse(self) -> Self;
    }

    pub trait Value {
        type T;
        fn value(&mut self) -> &Self::T;
    }
    pub trait ValueMut {
        type T;
        fn value_mut(&mut self) -> &mut Self::T;
    }

    pub trait Weight {
        type T;
        fn weight(&self) -> &Self::T;
    }
    pub trait WeightMut: Weight {
        fn weight_mut(&mut self) -> &mut Self::T;
    }

    pub trait Capacity<T> {
        fn capacity(&self) -> &T;
    }
}

pub mod dijkstra_sparse_general_extended {
    use crate::{
        graph_edge_trait::{To, Weight},
        priority_queue_trait::{Pop, Push},
    };

    /// T is a numeric type.
    pub fn dijkstra<Q, T, E, F>(
        adj_list: &[Vec<E>],
        src: usize,
        callback: &mut F,
    ) -> Vec<Option<T>>
    where
        Q: Push<T = (T, usize)> + Pop<T = Option<(T, usize)>> + Default,
        T: Default + Ord + Copy + std::ops::Add<Output = T>,
        E: To<V = usize> + Weight<T = T>,
        F: FnMut(&Vec<Option<T>>, usize, &E),
    {
        let g = adj_list;
        let n = g.len();
        let mut hq = Q::default();
        let mut dist: Vec<Option<T>> = vec![None; n];
        dist[src] = Some(T::default());
        hq.push((dist[src].unwrap(), src));
        while let Some((du, u)) = hq.pop() {
            if du > dist[u].unwrap() {
                continue;
            }
            for e in g[u].iter() {
                callback(&dist, u, e);
                let v = *e.to();
                let dv = du + *e.weight();
                if dist[v].is_some() && Some(dv) >= dist[v] {
                    continue;
                }
                dist[v] = Some(dv);
                hq.push((dv, v));
            }
        }
        dist
    }
}

pub mod dijkstra_sparse_path_count {
    use crate::{
        graph_edge_trait::{To, Weight},
        priority_queue_trait::{Pop, Push},
    };
    pub fn dijkstra<Q, T, E>(
        adj_list: &[Vec<E>],
        src: usize,
    ) -> (Vec<Option<T>>, Vec<u64>)
    where
        Q: Push<T = (T, usize)> + Pop<T = Option<(T, usize)>> + Default,
        T: Default + Ord + Copy + std::ops::Add<Output = T>,
        E: To<V = usize> + Weight<T = T>,
    {
        use crate::dijkstra_sparse_general_extended::dijkstra as f;
        const MOD: u64 = 1_000_000_007;
        let n = adj_list.len();
        let mut cnt = vec![0; n];
        cnt[src] = 1;
        let dist = f::<Q, _, _, _>(
            adj_list,
            src,
            &mut |d: &Vec<Option<T>>, u: usize, e: &E| {
                let v = *e.to();
                let dv = d[u].unwrap() + *e.weight();
                if d[v].is_none() || Some(dv) < d[v] {
                    cnt[v] = cnt[u];
                } else if Some(dv) == d[v] {
                    cnt[v] = (cnt[v] + cnt[u]) % MOD;
                }
            },
        );
        (dist, cnt)
    }
}

pub struct BinaryMinHeap<T>(Vec<T>);

impl<T> BinaryMinHeap<T> {
    pub fn new() -> Self { Self(vec![]) }

    pub fn size(&self) -> usize { self.0.len() }

    pub fn push(&mut self, x: T)
    where
        T: PartialOrd,
    {
        let mut i = self.size();
        self.0.push(x);
        while i > 0 {
            let j = (i - 1) >> 1;
            if self.0[i] >= self.0[j] {
                break;
            }
            self.0.swap(i, j);
            i = j;
        }
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: PartialOrd,
    {
        let mut n = self.size();
        if n == 0 {
            return None;
        }
        self.0.swap(0, n - 1);
        let x = self.0.pop();
        n -= 1;
        let mut i = 0;
        loop {
            let mut j = (1 << i) | 1;
            if j >= n {
                break;
            }
            if j < n - 1 && self.0[j + 1] < self.0[j] {
                j += 1;
            }
            if self.0[i] <= self.0[j] {
                break;
            }
            self.0.swap(i, j);
            i = j;
        }
        x
    }
}

impl<T> Default for BinaryMinHeap<T> {
    fn default() -> Self { Self::new() }
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

    type E = (usize, u64);
    use crate::graph_edge_trait::{To, Weight};

    impl To for E {
        type V = usize;

        fn to(&self) -> &usize { &self.0 }
    }

    impl Weight for E {
        type T = u64;

        fn weight(&self) -> &u64 { &self.1 }
    }

    use std::cmp::Reverse;
    // type Q<T> = std::collections::BinaryHeap<Reverse<T>>;
    type Q<T> = BinaryMinHeap<Reverse<T>>;
    use crate::priority_queue_trait::{Pop, Push};
    impl<T: Ord> Push for Q<T> {
        type T = T;

        fn push(&mut self, x: T) { Self::push(self, Reverse(x)); }
    }

    impl<T: Ord> Pop for Q<T> {
        type T = Option<T>;

        fn pop(&mut self) -> Self::T {
            if let Some(Reverse(x)) = Self::pop(self) {
                Some(x)
            } else {
                None
            }
        }
    }
    let (dist, cnt) =
        dijkstra_sparse_path_count::dijkstra::<Q<(u64, usize)>, _, _>(&g, src);
    // dbg!(&cnt);
    for &d in dist.iter() {
        if let Some(d) = d {
            writeln!(writer, "{}", d).unwrap();
        } else {
            writeln!(writer, "INF").unwrap();
        }
    }
}

