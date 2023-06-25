#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
        xyr: [(i64, i64, i64); n]
    }
    let mut g = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let (xi, yi, ri) = xyr[i];
            let (xj, yj, rj) = xyr[j];
            let d = (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj);
            if d > (ri + rj) * (ri + rj) || d < (ri - rj) * (ri - rj) {
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
    for (i, xyri) in xyr.iter().enumerate().take(n) {
        let (xi, yi, ri) = xyri;
        if ri * ri == (xi - sx) * (xi - sx) + (yi - sy) * (yi - sy) {
            s.push(i);
        }

        if ri * ri == (xi - tx) * (xi - tx) + (yi - ty) * (yi - ty) {
            t.push(i);
        }
    }
    for k in 0..s.len() {
        let mut q = VecDeque::new();
        q.push_back(s[k]);
        let mut visited = vec![false; n];
        visited[s[k]] = true;
        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            for (j, visitedj) in visited.iter_mut().enumerate().take(n) {
                if g[i][j] == 1 && !*visitedj {
                    q.push_back(j);
                    *visitedj = true;
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
