pub struct Scanner<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    /// let stdin = std::io::stdin();
    /// let mut sc = Scanner::new(stdin.lock());
    pub fn new(reader: R) -> Self { Self { reader: reader } }

    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c|c.unwrap()as char)
        .skip_while(|c|c.is_whitespace())
        .take_while(|c|!c.is_whitespace())
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
    let mut g: Vec<(usize, usize, i64)> = Vec::with_capacity(m);
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let d: i64 = sc.scan();
        g.push((s, t, d));
    }
    let mst = boruvka(n, &g);
    writeln!(out, "{}", mst.iter().map(|x| x.2).sum::<i64>()).unwrap();
}



pub struct UnionFind {
    data: Vec<i32>,
}


impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { data: vec![-1; n] }
    }
    
    pub fn find(&mut self, u: usize) -> usize {
        if self.data[u] < 0 { return u; }
        self.data[u] = self.find(self.data[u] as usize) as i32;
        self.data[u] as usize
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let (mut u, mut v) = (self.find(u), self.find(v));
        if u == v { return; }
        if self.data[u] > self.data[v] { std::mem::swap(&mut u, &mut v); }
        self.data[u] += self.data[v];
        self.data[v] = u as i32;
    }

    pub fn size(&mut self, u: usize) -> usize {
        let u = self.find(u);
        -self.data[u] as usize
    }
}


mod connected_components {
    use super::*;


    pub fn with_dfs(n: usize, g: &Vec<(usize, usize)>) -> Vec<usize> {
        let mut t: Vec<Vec<usize>> = vec![vec![]; n];
        for (u, v) in g.iter() {
            t[*u].push(*v);
            t[*v].push(*u);
        }
        let mut label = vec![n; n];
        let mut l = 0 as usize;
        fn dfs(u: usize, label: &mut Vec<usize>, l: usize, t: &Vec<Vec<usize>>) -> () {
            label[u] = l;
            for v in t[u].iter() {
                if label[*v] == t.len() { dfs(*v, label, l, t); } 
            }
        }
        for i in 0..n {
            if label[i] != n { continue; }
            dfs(i, &mut label, l, &t);
            l += 1;
        }
        label
    }

    pub fn with_bfs(n: usize, g: &Vec<(usize, usize)>) -> Vec<usize> {
        let mut t: Vec<Vec<usize>> = vec![vec![]; n];
        for (u, v) in g.iter() {
            t[*u].push(*v);
            t[*v].push(*u);
        }
        let mut label = vec![n; n];
        let mut l = 0 as usize;
        for i in 0..n {
            if label[i] != n { continue; }
            label[i] = l;
            let mut que = std::collections::VecDeque::new();
            que.push_back(i);
            while let Some(u) = que.pop_front() {
                for v in t[u].iter() {
                    if label[*v] != n { continue; }
                    label[*v] = l;
                    que.push_back(*v);   
                }
            }
            l += 1;
        }
        label
    }

    pub fn with_union_find(n: usize, g: &Vec<(usize, usize)>) -> Vec<usize> {
        let mut label = vec![n; n];
        let mut l = 0 as usize;
        let mut uf = UnionFind::new(n);
        for (u, v) in g { uf.unite(*u, *v); }
        for i in 0..n {
            let root = uf.find(i);
            if label[root] == n { label[root] = l; l += 1 }
            label[i] = label[root];
        }
        label
    }
}


pub fn boruvka(n: usize, g: &Vec<(usize, usize, i64)>) -> Vec<(usize, usize, i64)> {
    let m = g.len();
    let mut is_added = vec![false; g.len()];
    let mut mst: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    loop {
        let label = connected_components::with_bfs(n, &mst.iter().map(|x| (x.0, x.1)).collect());
        let k = *label.iter().max().unwrap() + 1;
        if k == 1 { break; }
        let mut min_edge = vec![m; k];
        for (i, (u, v, w)) in g.iter().enumerate() {
            let u = label[*u];
            let v = label[*v];
            if u == v { continue; }
            if min_edge[u] == m || *w < g[min_edge[u]].2 { min_edge[u] = i; }
            if min_edge[v] == m || *w < g[min_edge[v]].2 { min_edge[v] = i; }         
        }
        for i in min_edge {
            if is_added[i] { continue; }
            mst.push(g[i]);
            is_added[i] = true;  
        }
    }
    mst
}

