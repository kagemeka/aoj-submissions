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

    let inf = std::i64::MAX;
    let n: usize = sc.scan();
    let mut g: Vec<(usize, usize)> = Vec::with_capacity(n - 1);
    for u in 0..n {
        let k: usize = sc.scan();
        for _ in 0..k {
            let v: usize = sc.scan();
            g.push((u, v));
        }
    }

    let lca = eulertour_rmq::WithSparseTable::new(&g, 0);
    let q: usize = sc.scan();
    for _ in 0..q {
        let u: usize = sc.scan();
        let v: usize = sc.scan();
        writeln!(out, "{}", lca.get(u, v)).unwrap();
    }
}


pub mod eulertour_rmq {
    use super::{SparseTable, euler_tour_node, Semigroup};


    pub struct WithSparseTable<S> {
        first_idx: Vec<usize>,
        sp: SparseTable<S>,
    }
    impl WithSparseTable<(usize, usize)> {
        pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
            let (tour, first_idx, _, depth) = euler_tour_node(g, root);
            let sg = Semigroup::<(usize, usize)> {
                op: Box::new(|x, y| std::cmp::min(*x, *y)),
                commutative: true,
                idempotent: true,
            };
            let mut a = Vec::with_capacity(tour.len());
            for &i in tour.iter() {
                a.push((depth[i as usize], i as usize));
            }
            let sp = SparseTable::new(sg, &a);
            Self { first_idx: first_idx, sp: sp }
        }

        pub fn get(&self, u: usize, v: usize) -> usize {
            let mut l = self.first_idx[u];
            let mut r = self.first_idx[v];
            if l > r { std::mem::swap(&mut l, &mut r); }
            self.sp.get(l, r).1
        }
    }



    pub struct WithSegtree {
    }

}



pub fn euler_tour_edge(g: &Vec<(usize, usize)>, root: usize) -> (Vec<isize>, Vec<usize>, Vec<usize>) {
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for &(u, v) in g.iter() {
        t[u].push(v);
        t[v].push(u);
    }
    let mut parent = vec![n; n];
    let mut depth = vec![0; n];
    let mut tour = Vec::with_capacity(n << 1);
    let mut st = vec![root as isize];
    for _ in 0..n << 1 {
        let u = st.pop().unwrap();
        tour.push(u);
        if u < 0 { continue; }
        st.push(!u);
        let u = u as usize;
        for &v in t[u].iter() {
            if v == parent[u] { continue; }
            parent[v] = u;
            depth[v] = depth[u] + 1;
            st.push(v as isize);
        }
    }
    (tour, parent, depth)
}


pub fn euler_tour_node(g: &Vec<(usize, usize)>, root: usize) -> (Vec<isize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let (mut tour, parent, depth) = euler_tour_edge(g, root);
    let n = tour.len() >> 1;
    tour.pop();
    let mut first_idx = vec![n; n];
    for i in 0..tour.len() {
        let u = tour[i];
        if u < 0 { tour[i] = parent[!u as usize] as isize; continue; }
        first_idx[u as usize] = i;
    }
    (tour, first_idx, parent, depth)
}


/// O(1)
pub fn bit_length(n: usize) -> usize {
    (0usize.leading_zeros() - n.leading_zeros()) as usize
}

pub struct Semigroup<S> {
    pub op: Box<dyn Fn(&S, &S) -> S>,
    pub commutative: bool,
    pub idempotent: bool,
}


/// Sparse Table
/// references
/// - https://cp-algorithms.com/data_structures/sparse-table.html
pub struct SparseTable<S> {
    sg: Semigroup<S>,
    data: Vec<Vec<S>>,
}


impl<S: Default + Clone> SparseTable<S> {
    /// O(N\log{N})
    pub fn new(sg: Semigroup<S>, a: &Vec<S>) -> Self {
        assert!(sg.idempotent && sg.commutative);
        let n = a.len();
        assert!(n > 0);
        let k = std::cmp::max(1, bit_length(n - 1));
        let mut data = vec![vec![S::default(); n]; k];
        data[0] = a.clone();
        for i in 0..k - 1 {
            data[i + 1] = data[i].clone();
            for j in 0..n - (1 << i) {
                data[i + 1][j] = (sg.op)(&data[i][j], &data[i][j + (1 << i)])
            }
        }
        Self { sg: sg, data: data }
    }

    /// O(1)
    pub fn get(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.data[0].len());
        let k = bit_length(r - l) - 1;
        (self.sg.op)(&self.data[k][l], &self.data[k][r - (1 << k)])
    }
}




/// Disjoint Sparse Table
/// references
/// - https://noshi91.hatenablog.com/entry/2018/05/08/183946
/// - https://github.com/noshi91/NoshiMochiLibrary/blob/master/SparseTable/DisjointSparseTable.noshi.cpp
pub struct DisjointSparseTable<'a, S> {
    sg: &'a Semigroup<S>,
    data: Vec<Vec<S>>,
}


impl<'a, S: Default + Clone> DisjointSparseTable<'a, S> {
    pub fn new(sg: &'a Semigroup<S>, a: &Vec<S>) -> Self {
        let n = a.len();
        assert!(n > 0);
        let k = std::cmp::max(1, bit_length(n - 1));
        let mut data = vec![vec![S::default(); n]; k];
        data[0] = a.clone();
        for i in 1..k {
            data[i] = a.clone();
            for j in (1 << i..n + 1).step_by(2 << i) {
                for k in 1..(1 << i) {
                    data[i][j - k - 1] = (sg.op)(&data[i][j - k - 1], &data[i][j - k]);
                }
                for k in 0..(1 << i) - 1 {
                    if j + k + 1 >= n { break; }
                    data[i][j + k + 1] = (sg.op)(&data[i][j + k], &data[i][j + k + 1]);
                }
            }
        }
        Self { sg: sg, data: data }
    }

    pub fn get(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.data[0].len());
        if r - l == 1 { return self.data[0][l].clone(); }
        let k = bit_length(l ^ r) - 1;
        (self.sg.op)(&self.data[k][l], &self.data[k][r - 1])
    }
}
