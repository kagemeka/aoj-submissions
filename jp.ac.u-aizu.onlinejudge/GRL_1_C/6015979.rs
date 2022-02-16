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

    let n: usize = sc.scan();
    let m: usize = sc.scan();
    let inf = std::i64::MAX;
    let mut g: Vec<Vec<i64>> = vec![vec![inf; n]; n];
    for _ in 0..m {
        let s: usize = sc.scan();
        let t: usize = sc.scan();
        let d: i64 = sc.scan();
        g[s][t] = std::cmp::min(g[s][t], d);
    }
    for i in 0..n { g[i][i] = 0; }
    if let Ok(dist) = johnson_dense(&g) {
       for i in 0..n {
           for j in 0..n {
               if dist[i][j] == inf { 
                   write!(out, "INF").unwrap();
               } else {
                   write!(out, "{}", dist[i][j]).unwrap();
               }
               write!(out, "{}", if j < n - 1 { ' ' } else { '\n' }).unwrap();
           }
        }
    } else {
        writeln!(out, "NEGATIVE CYCLE").unwrap();
    }
}



pub fn johnson_dense(g: &Vec<Vec<i64>>) -> Result<Vec<Vec<i64>>, NegativeCycleError> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut t = g.to_vec();
    t.push(vec![0; n + 1]);
    for i in 0..n { t[i].push(inf); }
    let h = bellman_ford_dense(&t, n)?;
    t = g.to_vec();
    for u in 0..n {
        for v in 0..n {
            if t[u][v] == inf { continue; }
            t[u][v] += h[u] - h[v];
        }
    }
    let mut dist = Vec::with_capacity(n);
    for i in 0..n {
        let mut d = dijkstra_dense(&t, i);
        for j in 0..n { 
            if d[j] != inf { d[j] += h[j] - h[i]; } 
        }
        dist.push(d);
    }
    Ok(dist)
}


pub fn dijkstra_dense(g: &Vec<Vec<i64>>, src: usize) -> Vec<i64> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    let mut visited = vec![false; n];
    loop {
        let mut u = -1;
        let mut du = inf;
        for i in 0..n {
            if !visited[i] && dist[i] < du { u = i as i32; du = dist[i]; }
        }
        if u == -1 { break; }
        let u = u as usize;
        visited[u] = true;
        for v in 0..n {
            if g[u][v] == inf { continue; }
            let dv = du + g[u][v];
            if dv >= dist[v] { continue; }
            dist[v] = dv;
        }
    }
    dist
}



#[derive(Debug)]
pub struct NegativeCycleError {
    msg: &'static str,
}

impl NegativeCycleError {
    fn new() -> Self {
        Self { msg: "Negative Cycle Found." }
    }  
}

impl std::fmt::Display for NegativeCycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NegativeCycleError {
    fn description(&self) -> &str { &self.msg }
}

/// O(v^3)
pub fn bellman_ford_dense(g: &Vec<Vec<i64>>, src: usize) -> Result<Vec<i64>, NegativeCycleError> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    for _ in 0..n - 1 {
        for u in 0..n {
            for v in 0..n {
                if dist[u] == inf || g[u][v] == inf || dist[u] + g[u][v] >= dist[v] { continue; }
                dist[v] = dist[u] + g[u][v];
            }
        }
    }
    for u in 0..n {
        for v in 0..n {
            if dist[u] == inf || g[u][v] == inf || dist[u] + g[u][v] >= dist[v] { continue; }
            return Err(NegativeCycleError::new());
        }
    }
    Ok(dist)
}
