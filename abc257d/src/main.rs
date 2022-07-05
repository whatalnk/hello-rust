use proconio::input;
use std::collections::VecDeque;

struct S {
    n: usize,
    xyp: Vec<(i64, i64, i64)>,
}
impl S {
    fn f(&self, st: usize, s: i64) -> bool {
        let mut g = vec![vec![0; self.n]; self.n];
        let mut done = vec![false; self.n];
        let mut q = VecDeque::new();
        q.push_back(st);
        done[st] = true;
        while q.len() > 0 {
            let i = q.pop_front().unwrap();
            for j in 0..self.n {
                if g[i][j] == 0 && !done[j] {
                    let (xi, yi, pi) = self.xyp[i];
                    let (xj, yj, pj) = self.xyp[j];
                    if s * pi - ((xi - xj).abs() + (yi - yj).abs()) >= 0 {
                        g[i][j] = 1;
                        done[j] = true;
                        q.push_back(j);
                    }
                }
            }
        }
        return done.iter().all(|&x| x);
    }
}

fn main() {
    input! {
        n: usize,
        xyp: [(i64, i64, i64); n]
    }
    let s = S { n: n, xyp: xyp };
    for i in 0..n {
        for j in 0..5 {
            println!("{} {} {}", i, j, s.f(i, j));
        }
    }
}
