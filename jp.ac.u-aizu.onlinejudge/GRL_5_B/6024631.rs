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

    // let inf = std::i64::MAX;
    let n: usize = sc.scan();
    let mut g: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    for _ in 0..n - 1 {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let w: i64 = sc.scan();
        g.push((s, t, w));
    }
    let m = Monoid::<i64> {
        op: Box::new(|x, y| std::cmp::max(*x, *y)) ,
        e: Box::new(|| 0),
        commutative: true,
    };
    let map = |x: &i64, y: &i64| x + y;
    let dp = rerooting(&g, &m, Box::new(map));
    for x in dp {
        writeln!(out, "{}", x).unwrap();
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



/// Monoid<S> is Commutative.
/// map: F x S -> S is homomorphism.
pub fn rerooting<S: Clone, F>(g: &Vec<(usize, usize, F)>, m: &Monoid<S>, map: Box<dyn Fn(&F, &S) -> S>) -> Vec<S> {
    fn tree_dp<S, F>(
        g: &Vec<Vec<(usize, &F)>>,
        dp: &mut Vec<S>,
        m: &Monoid<S>,
        map: &Box<dyn Fn(&F, &S) -> S>,
        u: usize,
        parent: usize,
    ) {
        for &(v, x) in g[u].iter() {
            if v == parent { continue; }
            tree_dp(g, dp, m, map, v, u);
            dp[u] = (m.op)(&dp[u], &map(x, &dp[v]));
        }
    }
    fn reroot<S: Clone, F>(
        g: &Vec<Vec<(usize, &F)>>,
        dp0: &Vec<S>,
        dp1: &mut Vec<S>,
        m: &Monoid<S>,
        map: &Box<dyn Fn(&F, &S) -> S>,
        u: usize,
        parent: usize,
    ) {
        let mut childs = Vec::new();
        for &e in g[u].iter() {if e.0 != parent { childs.push(e); }}
        let deg = childs.len();
        let mut dp_l = vec![(m.e)(); deg + 1];
        let mut dp_r = vec![(m.e)(); deg + 1];
        for (i, &(v, x)) in childs.iter().enumerate() {
            dp_l[i + 1] = (m.op)(&dp_l[i], &map(x, &dp0[v]));
        }
        for (i, &(v, x)) in childs.iter().enumerate().rev() {
            dp_r[i] = (m.op)(&dp_r[i + 1], &map(x, &dp0[v]));
        }
        for (i, &(v, x)) in childs.iter().enumerate() {
            dp1[v] = map(x, &(m.op)(&dp1[u], &(m.op)(&dp_l[i], &dp_r[i + 1])));
            reroot(g, dp0, dp1, m, map, v, u);
        }

    }
    assert_eq!(m.commutative, true);
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for (u, v, x) in g.iter() {
        t[*u].push((*v, x));
        t[*v].push((*u, x));
    }
    let mut dp0: Vec<S> = vec![(m.e)(); n];
    let mut dp1: Vec<S> = vec![(m.e)(); n];
    tree_dp(&t, &mut dp0, m, &map, 0, n);
    reroot(&t, &dp0, &mut dp1, m, &map, 0, n);
    let mut dp = vec![(m.e)(); n];
    for i in 0..n {
        dp[i] = (m.op)(&dp0[i], &dp1[i]);
    }
    dp
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
        st.push(-(u + 1));
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
        if u < 0 { tour[i] = parent[-(u + 1) as usize] as isize; continue; }
        first_idx[u as usize] = i;
    }
    (tour, first_idx, parent, depth)
}
