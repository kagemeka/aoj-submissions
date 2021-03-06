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
    let m: usize = sc.scan();
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        g[s].push(t);
    }
    let label = path_based(&g);
    let q: usize = sc.scan();
    for _ in 0..q {
        let u: usize = sc.scan();
        let v: usize = sc.scan();
        writeln!(out, "{:?}", if label[u] == label[v] { 1 } else { 0 }).unwrap();
    }
}



/// scc path based algorithm
/// references
/// - https://en.wikipedia.org/wiki/Path-based_strong_component_algorithm
pub fn path_based(g: &Vec<Vec<usize>>) -> Vec<usize> {
    fn dfs(
        g: &Vec<Vec<usize>>,
        order: &mut Vec<usize>,
        label: &mut Vec<usize>,
        st_0: &mut Vec<usize>,
        st_1: &mut Vec<usize>,
        u: usize,
        ord: &mut usize,
        l: &mut usize,
    ) {
        order[u] = *ord;
        *ord += 1;
        st_0.push(u);
        st_1.push(u);
        for v in g[u].iter().map(|x| *x) {
            if order[v] == g.len() {
                dfs(g, order, label, st_0, st_1, v, ord, l);
            } else if label[v] == g.len() {
                while order[*st_0.last().unwrap()] > order[v] { st_0.pop(); }
            }
        }
        if *st_0.last().unwrap() == u {
            loop {
                let v = st_1.pop().unwrap();
                label[v] = *l;
                if v == u { break; }
            }
            *l += 1;
            st_0.pop();
        }
    }
    let n = g.len();
    let mut order = vec![n; n];
    let mut label = vec![n; n];
    let mut st_0 = Vec::with_capacity(n);
    let mut st_1 = Vec::with_capacity(n);
    let mut ord = 0;
    let mut l = 0;
    for i in 0..n {
        if order[i] == n { dfs(g, &mut order, &mut label, &mut st_0, &mut st_1, i, &mut ord, &mut l); }
    }
    label
}


/// scc Tarjan with lowlink
/// references
/// - https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
pub fn tarjan(g: &Vec<Vec<usize>>) -> Vec<usize> {
    fn dfs(
        g: &Vec<Vec<usize>>,
        order: &mut Vec<usize>,
        low: &mut Vec<usize>,
        label: &mut Vec<usize>,
        on_stack: &mut Vec<bool>,
        st: &mut Vec<usize>,
        u: usize,
        ord: &mut usize,
        l: &mut usize,
    ) {
        order[u] = *ord;
        low[u] = *ord;
        *ord += 1;
        st.push(u);
        on_stack[u] = true;
        for v in g[u].iter().map(|x| *x) {
            if order[v] == g.len() {
                dfs(g, order, low, label, on_stack, st, v, ord, l);
                if low[v] < low[u] { low[u] = low[v]; }
            } else if on_stack[v] && order[v] < low[u] {
                low[u] = order[v];
            }
        }
        if low[u] == order[u] {
            loop {
                let v = st.pop().unwrap();
                on_stack[v] = false;
                label[v] = *l;
                if v == u { break; }
            }
            *l += 1;
        }
    }
    let n = g.len();
    let mut order = vec![n; n];
    let mut low = vec![n; n];
    let mut label = vec![n; n];
    let mut on_stack = vec![false; n];
    let mut st = Vec::with_capacity(n);
    let mut ord = 0;
    let mut l = 0;
    for i in 0..n {
        if order[i] == n { dfs(g, &mut order, &mut low, &mut label, &mut on_stack, &mut st, i, &mut ord, &mut l); }
    }
    label
}



/// scc Kosaraju
/// references
/// - https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
pub fn kosaraju(g: &Vec<Vec<usize>>) -> Vec<usize> {
    fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, que: &mut Vec<usize>, u: usize) {
        visited[u] = true;
        for v in g[u].iter() {
            if !visited[*v] { dfs(g, visited, que, *v); }
        }
        que.push(u);
    }
    fn rev_dfs(g: &Vec<Vec<usize>>, label: &mut Vec<usize>, l: usize, u: usize) {
        label[u] = l;
        for v in g[u].iter() {
            if label[*v] == g.len() { rev_dfs(g, label, l, *v); }
        }
    }
    let n = g.len();
    let mut visited = vec![false; n];
    let mut que = Vec::with_capacity(n);
    let mut label = vec![n; n];
    let mut l = 0usize;
    for i in 0..n {
        if !visited[i] { dfs(g, &mut visited, &mut que, i); }
    }
    let mut t = vec![vec![]; n];
    for u in 0..n {
        for v in g[u].iter() { t[*v].push(u); }
    }
    for i in que.iter().rev() {
        if label[*i] != n { continue; }
        rev_dfs(&t, &mut label, l, *i);
        l += 1;
    }
    label
}
