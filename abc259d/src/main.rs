use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        sx: f64,
        sy: f64,
        tx: f64,
        ty: f64,
        xyr: [(f64, f64, f64); n]
    }
    let mut g = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let (xi, yi, ri) = xyr[i];
            let (xj, yj, rj) = xyr[j];
            let d = ((xi - xj) * (xi - xj) + (yi - yj) * (yi - yj)).sqrt();
            if d > ri + rj || d < (ri - rj).abs() {
                g[i][j] = 0;
                g[j][i] = 0;
            } else {
                g[i][j] = 1;
                g[j][i] = 1;
            }
        }
    }
    let mut s = vec![];
    let mut t = vec![];
    for i in 0..n {
        let (xi, yi, ri) = xyr[i];
        if (ri as i64) * (ri as i64)
            == ((xi - sx) as i64) * ((xi - sx) as i64) + ((yi - sy) as i64) * ((yi - sy) as i64)
        {
            s.push(i);
        }

        if (ri as i64) * (ri as i64)
            == ((xi - tx) as i64) * ((xi - tx) as i64) + ((yi - ty) as i64) * ((yi - ty) as i64)
        {
            t.push(i);
        }
    }
    for k in 0..s.len() {
        let mut q = VecDeque::new();
        q.push_back(s[k]);
        let mut visited = vec![false; n];
        visited[s[k]] = true;
        while q.len() > 0 {
            let i = q.pop_front().unwrap();
            for j in 0..n {
                if g[i][j] == 1 && !visited[j] {
                    q.push_back(j);
                    visited[j] = true;
                }
            }
        }
        for i in 0..t.len() {
            if visited[t[i]] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
