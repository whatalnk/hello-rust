use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[ab[i].0 - 1].push(ab[i].1 - 1);
    }
    let mut ans = 0;
    for i in 0..n {
        let mut visited = vec![0; n];
        visited[i] = 1;
        ans += 1;
        let mut que = VecDeque::new();
        que.push_back(i);
        while !que.is_empty() {
            let cur = que.pop_front().unwrap();
            for j in &g[cur] {
                if visited[*j] == 0 {
                    visited[*j] = 1;
                    ans += 1;
                    que.push_back(*j);
                }
            }
        }
    }
    println!("{}", ans);
}
