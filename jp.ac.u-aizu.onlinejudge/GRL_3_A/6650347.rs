pub struct ReadWrapper<R> {
    reader: R,
    tokens: Vec<String>,
}

impl<R> ReadWrapper<R> {
    pub fn new(reader: R) -> Self { Self { reader, tokens: vec![] } }
}

impl<R: std::io::BufRead> ReadWrapper<R> {
    pub fn read<T: std::str::FromStr>(
        &mut self,
    ) -> Result<T, <T as std::str::FromStr>::Err> {
        while self.tokens.is_empty() {
            let mut buf = String::new();
            self.reader.read_line(&mut buf).unwrap();
            self.tokens =
                buf.split_whitespace().map(str::to_string).rev().collect();
        }
        self.tokens.pop().unwrap().parse::<T>()
    }
}

pub fn locked_stdin_reader() -> ReadWrapper<std::io::StdinLock<'static>> {
    let stdin = Box::leak(Box::new(std::io::stdin()));
    ReadWrapper::new(stdin.lock())
}

pub fn locked_stdin_buf_writer()
-> std::io::BufWriter<std::io::StdoutLock<'static>> {
    let stdout = Box::leak(Box::new(std::io::stdout()));
    std::io::BufWriter::new(stdout.lock())
}

// #[allow(warnings)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let mut reader = locked_stdin_reader();
    let mut writer = locked_stdin_buf_writer();

    let n: usize = reader.read()?;
    let m: usize = reader.read()?;

    let edges = (0..m)
        .map(|_| {
            let u: usize = reader.read().unwrap();
            let v: usize = reader.read().unwrap();
            (u, v)
        })
        .collect::<Vec<_>>();

    // let mut bridges = find_bridges_lowlink(n, &edges)
    //     .into_iter()
    //     .map(|i| {
    //         let (mut u, mut v) = edges[i];
    //         if u > v {
    //             std::mem::swap(&mut u, &mut v);
    //         }
    //         (u, v)
    //     })
    //     .collect::<Vec<_>>();
    // bridges.sort();

    // for (u, v) in bridges {
    //     writeln!(writer, "{} {}", u, v)?;
    // }
    let mut points = find_articulation_points_lowlink(n, &edges);
    points.sort();
    for p in points {
        writeln!(writer, "{}", p)?;
    }

    writer.flush()?;
    Ok(())
}

/// return vertex ids.
pub fn find_articulation_points_lowlink(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut articulation_points = Vec::new();
    let lowlink = undirected_lowlink(v_size, undirected_edges);
    let order = lowlink.orders;
    let low = lowlink.low_orders;

    let mut visited = vec![false; v_size];

    let mut g = vec![vec![]; v_size];
    for &(u, v) in undirected_edges {
        g[u].push(v);
        g[v].push(u);
    }
    fn find(
        g: &[Vec<usize>],
        order: &[usize],
        low: &[usize],
        points: &mut Vec<usize>,
        visited: &mut Vec<bool>,
        u: usize,
        parent: usize,
    ) {
        let n = g.len();
        visited[u] = true;
        let mut childs_count = 0;
        let mut is_articulation = false;
        for &v in &g[u] {
            if visited[v] {
                continue;
            }
            childs_count += 1;
            find(
                g, order, low, points, visited, v, u,
            );
            if parent != n && low[v] >= order[u] {
                is_articulation = true;
            }
        }
        if parent == n && childs_count > 1 {
            is_articulation = true;
        }
        if is_articulation {
            points.push(u);
        }
    }
    for i in 0..v_size {
        if visited[i] {
            continue;
        }
        find(
            &g,
            &order,
            &low,
            &mut articulation_points,
            &mut visited,
            i,
            v_size,
        );
    }
    articulation_points
}

/// return edge ids.
pub fn find_bridges_lowlink(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let lowlink = undirected_lowlink(v_size, undirected_edges);
    let order = lowlink.orders;
    let low = lowlink.low_orders;
    (0..undirected_edges.len())
        .filter(|&i| {
            let (mut u, mut v) = undirected_edges[i];
            if order[u] > order[v] {
                std::mem::swap(&mut u, &mut v);
            }
            low[v] == order[v]
        })
        .collect()
}

#[derive(Debug)]
pub struct LowlinkResult {
    orders: Vec<usize>,
    low_orders: Vec<usize>,
}

impl LowlinkResult {
    pub fn get_lowlinks(data: &LowlinkResult) -> Vec<usize> {
        let n = data.orders.len();
        let mut vertices = vec![0; n];
        for i in 0..n {
            vertices[data.orders[i]] = i;
        }
        data.low_orders.iter().map(|&i| vertices[i]).collect()
    }
}

pub fn undirected_lowlink(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> LowlinkResult {
    let mut g = vec![vec![]; v_size];
    for (i, &(u, v)) in undirected_edges.iter().enumerate() {
        g[u].push((v, i));
        g[v].push((u, i));
    }

    let mut orders = vec![v_size; v_size];
    let mut order = 0;
    let mut low_orders = vec![v_size; v_size];

    fn compute_low_order(
        g: &[Vec<(usize, usize)>],
        orders: &mut [usize],
        order: &mut usize,
        low_orders: &mut [usize],
        u: usize,
        edge_from: usize,
    ) {
        orders[u] = *order;
        low_orders[u] = *order;
        *order += 1;
        for &(v, eid) in &g[u] {
            if orders[v] == g.len() {
                compute_low_order(
                    g, orders, order, low_orders, v, eid,
                );
                low_orders[u] = std::cmp::min(low_orders[u], low_orders[v]);
            } else if eid != edge_from {
                low_orders[u] = std::cmp::min(low_orders[u], orders[v]);
            }
        }
    }

    for u in 0..v_size {
        if orders[u] == g.len() {
            compute_low_order(
                &g,
                &mut orders,
                &mut order,
                &mut low_orders,
                u,
                undirected_edges.len(),
            );
        }
    }

    LowlinkResult { orders, low_orders }
}
