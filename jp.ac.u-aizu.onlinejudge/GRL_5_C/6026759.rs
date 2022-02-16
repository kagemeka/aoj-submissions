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

    let lca = eulertour_rmq::WithSegmentTree::new(&g, 0);
    let q: usize = sc.scan();
    for _ in 0..q {
        let u: usize = sc.scan();
        let v: usize = sc.scan();
        writeln!(out, "{}", lca.get(u, v)).unwrap();
    }
}


pub mod eulertour_rmq {
    use super::{SegmentTree, Monoid, euler_tour_node};


    pub struct WithSegmentTree<S: Copy> {
        first_idx: Vec<usize>,
        seg: SegmentTree<S>,
    }
    impl WithSegmentTree<(usize, usize)> {
        pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
            let (tour, first_idx, _, depth) = euler_tour_node(g, root);
            let m = Monoid::<(usize, usize)> {
                op: Box::new(|x, y| std::cmp::min(*x, *y)),
                e: Box::new(|| (std::usize::MAX, 0)),
                commutative: true,
            };
            let mut a = Vec::with_capacity(tour.len());
            for &i in tour.iter() {
                a.push((depth[i as usize], i as usize));
            }
            let seg = SegmentTree::from_vec(m, &a);
            Self { first_idx: first_idx, seg: seg }
        }

        pub fn get(&self, u: usize, v: usize) -> usize {
            let mut l = self.first_idx[u];
            let mut r = self.first_idx[v];
            if l > r { std::mem::swap(&mut l, &mut r); }
            self.seg.get(l, r + 1).1
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




/// explicit lifetime for Monoid<S>.
/// S implements Copy trait for convenience.
pub struct SegmentTree<S: Copy> {
    m: Monoid<S>,
    data: Vec<S>,
    size: usize,
}


impl<'a, S: std::fmt::Debug + Copy> std::fmt::Debug for SegmentTree<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SegmentTree").field(&self.data).finish()
    }
}

impl<S: Copy> SegmentTree<S> {

    pub fn new(m: Monoid<S>, n: usize) -> Self {
        let a = vec![(m.e)(); n];
        Self::from_vec(m, &a)
    }

    pub fn from_vec(m: Monoid<S>, a: &Vec<S>) -> Self {
        let size = a.len();
        let n = size.next_power_of_two();
        let mut data = vec![(m.e)(); n << 1];
        for i in 0..size { data[n + i] = a[i]; }
        let mut seg = Self { m, data, size };
        for i in (0..n).rev() { seg.merge(i); }
        seg        
    }

    fn merge(&mut self, i: usize) {
        self.data[i] = (self.m.op)(&self.data[i << 1], &self.data[i << 1 | 1]);
    }

    pub fn set(&mut self, mut i: usize, x: S) {
        assert!(i < self.size);
        i += self.data.len() >> 1;
        self.data[i] = x;
        while i > 1 { i >>= 1; self.merge(i); }
    }

    pub fn get(&self, mut l: usize, mut r: usize) -> S {
        assert!(l <= r && r <= self.size);
        let n = self.data.len() >> 1;
        l += n; r += n;
        let mut vl = (self.m.e)();
        let mut vr = (self.m.e)();
        while l < r {
            if l & 1 == 1 { vl = (self.m.op)(&vl, &self.data[l]); l += 1; }
            if r & 1 == 1 { r -= 1; vr = (self.m.op)(&self.data[r], &vr); }
            l >>= 1; r >>= 1; 
        }
        (self.m.op)(&vl, &vr)
    }

    pub fn max_right(&self, is_ok: Box<dyn Fn(&S) -> bool>, l: usize) -> usize {
        assert!(l < self.size);
        let n = self.data.len() >> 1;
        let mut v = (self.m.e)();
        let mut i = (l + n) as i32;
        loop {
            i /= i & -i;
            if is_ok(&(self.m.op)(&v, &self.data[i as usize])) {
                v = (self.m.op)(&v, &self.data[i as usize]);
                i += 1;
                if i & -i == i { return self.size; }
                continue;
            }
            while i < n as i32 {
                i <<= 1;
                if is_ok(&(self.m.op)(&v, &self.data[i as usize])) {
                    v = (self.m.op)(&v, &self.data[i as usize]);
                }
            }
            return i as usize - n;
        }
    }
}


impl<S: Copy> std::ops::Index<usize> for SegmentTree<S> {
    type Output = S; 
    
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + (self.data.len() >> 1)]
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

