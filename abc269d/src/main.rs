use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }
    let dx = vec![-1, -1, 0, 0, 1, 1];
    let dy = vec![-1, 0, -1, 1, 0, 1];

    let mut black = HashSet::<(i64, i64)>::new();
    let mut visited = HashSet::<(i64, i64)>::new();

    for i in 0..n {
        black.insert(xy[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        if !visited.contains(&xy[i]) {
            ans += 1;
            let mut q = vec![];
            q.push(xy[i]);
            while q.len() > 0 {
                if let Some((x, y)) = q.pop() {
                    visited.insert((x, y));
                    for d in 0..6 {
                        let nx = x + dx[d];
                        let ny = y + dy[d];
                        if black.contains(&(nx, ny)) && !visited.contains(&(nx, ny)) {
                            q.push((nx, ny));
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
