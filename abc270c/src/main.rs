use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        uv: [(usize, usize); n-1]
    }
    // bfs
    let mut g = vec![vec![]; n + 1];
    for uvi in uv.iter().take(n - 1) {
        let (u, v) = uvi;
        g[*u].push(*v);
        g[*v].push(*u);
    }

    let inf: i64 = 1_000_000_000;
    let mut d = vec![inf; n + 1];
    d[x] = 0;
    let mut q = vec![];
    let mut prev = vec![-1; n + 1];
    q.push(x);
    while !q.is_empty() {
        if let Some(p) = q.pop() {
            if p == y {
                break;
            }
            for i in 0..g[p].len() {
                if d[g[p][i]] == inf {
                    q.push(g[p][i]);
                    d[g[p][i]] = d[p] + 1;
                    prev[g[p][i]] = p as isize;
                }
            }
        }
    }
    let mut path = vec![];
    path.push(y as isize);
    let mut t = prev[y];
    path.push(t);
    loop {
        t = prev[t as usize];
        if t == -1 {
            break;
        }
        path.push(t);
    }
    path.reverse();
    println!("{}", path.iter().map(|e| e.to_string()).join(" "));
}
