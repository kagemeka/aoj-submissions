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


    // keywords
    // - euler_tour 
    // - range add, point get
    //     - fenwick tree
    //     - segment tree dual
    //     - ... 
    // references
    // - https://perogram.hateblo.jp/entry/2020/10/01/034136

    let inf = std::i64::MAX;
    let n: usize = sc.scan();
    let mut g = Vec::with_capacity(n - 1);
    for u in 0..n {
        let k: usize = sc.scan();
        for _ in 0..k {
            let v: usize = sc.scan();
            g.push((u, v));
        }
    }
    let (_, first, last, _, _) = euler_tour_node(&g, 0);
    let m = Monoid::<i64> {
        op: Box::new(|x, y| x + y),
        e: Box::new(|| 0),
        commutative: true,
    };
    let mut fw = FenwickTree::new(m, 2 * n - 1);
    let q: usize = sc.scan();
    for _ in 0..q {
        let t: usize = sc.scan();
        if t == 0 {
            let u: usize = sc.scan();
            let w: i64 = sc.scan();
            fw.set(first[u], &w); 
            fw.set(last[u] + 1, &-w);
        } else {
            let u: usize = sc.scan();
            writeln!(out, "{}", fw.get(first[u] + 1)).unwrap();
        }
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


pub fn euler_tour_node(g: &Vec<(usize, usize)>, root: usize) -> (Vec<isize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let (mut tour, parent, depth) = euler_tour_edge(g, root);
    let n = tour.len() >> 1;
    tour.pop();
    let mut first_idx = vec![n; n];
    let mut last_idx = vec![n; n];
    for i in 0..tour.len() {
        let mut u = tour[i];
        if u < 0 { 
            u = parent[!u as usize] as isize;
            tour[i] = u; 
        } else {
            first_idx[u as usize] = i;
        }
        last_idx[u as usize] = i;
    }
    (tour, first_idx, last_idx, parent, depth)
}


pub mod extension {
    pub fn compute_first_idx(tour_edge: &Vec<isize>) -> Vec<usize> {
        let n = tour_edge.len() >> 1;
        let mut first_idx = vec![0; n];
        for (i, &u) in tour_edge.iter().enumerate() {
            if u >= 0 { first_idx[u as usize] = i; }
        }
        first_idx
    }

    // pub fn compute_last_idx(tour_edge: &Vec<isize>) -> Vec<usize> {
    //     let n = tour_edge.len() >> 1;
    //     let mut last_idx = vec![0; n];
    //     for (i, &u) in tour_edge.iter().enumerate() {
    //         if u < 0 { last_idx[u as usize] = i; }
    //     }
    //     last_idx
    // }
}



/// O(1)
pub fn bit_length(n: usize) -> usize {
    (0usize.leading_zeros() - n.leading_zeros()) as usize
}



pub struct FenwickTree<S: Copy> {
    m: Monoid<S>,
    data: Vec<S>,
}


impl<S: std::fmt::Debug + Copy> std::fmt::Debug for FenwickTree<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FenwickTree").field(&self.data).finish()
    }
}

impl<S: Copy> FenwickTree<S> {

    pub fn new(m: Monoid<S>, n: usize) -> Self {
        let a = vec![(m.e)(); n];
        Self::from_vec(m, &a)
    }

    pub fn from_vec(m: Monoid<S>, a: &Vec<S>) -> Self {
        let n = a.len();
        let mut data = vec![(m.e)(); n + 1];
        for i in 0..n { data[i + 1] = a[i]; }
        for i in 1..=n as i32 {
            let j = (i + (i & -i)) as usize;
            if j < n + 1 { data[j] = (m.op)(&data[j], &data[i as usize]); }
        }
        Self { m, data }
    }


    pub fn set(&mut self, mut i: usize, x: &S) {
        assert!(i < self.data.len() - 1);
        i += 1;
        while i < self.data.len() {
            self.data[i] = (self.m.op)(&self.data[i], x);
            i += (i as i32 & -(i as i32)) as usize;
        }
    }

    pub fn get(&self, mut i: usize) -> S {
        assert!(i < self.data.len());
        let mut v = (self.m.e)();
        while i > 0 {
            v = (self.m.op)(&v, &self.data[i]);
            i -= (i as i32 & -(i as i32)) as usize;
        }
        v
    }

    pub fn max_right(&self, is_ok: Box<dyn Fn(&S) -> bool>) -> usize {
        let n = self.data.len();
        let mut l = 1;
        while l << 1 < n { l <<= 1; }
        let mut v = (self.m.e)();
        let mut i = 0usize;
        while l > 0 {
            if i + l < n && is_ok(&(self.m.op)(&v, &self.data[i + l])) {
                i += l;
                v = (self.m.op)(&v, &self.data[i + 1]);
            }
            l >>= 1;
        }
        i
    }
}


// Fn(&S, &S) -> S is a trait.
/// this is a dynamic size object at compilation time.
/// thus, it's needed to be enclosed with Box<dyn> pointer.
pub struct Monoid<S> {
    pub op: Box<dyn Fn(&S, &S) -> S>,
    pub e: Box<dyn Fn() -> S>,
    pub commutative: bool,
}

