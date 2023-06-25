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
        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            for (j, donej) in done.iter_mut().enumerate().take(self.n) {
                if g[i][j] == 0 && !*donej {
                    let (xi, yi, pi) = self.xyp[i];
                    let (xj, yj, _) = self.xyp[j];
                    if s * pi - ((xi - xj).abs() + (yi - yj).abs()) >= 0 {
                        g[i][j] = 1;
                        *donej = true;
                        q.push_back(j);
                    }
                }
            }
        }
        done.iter().all(|&x| x)
    }
}

fn main() {
    input! {
        n: usize,
        xyp: [(i64, i64, i64); n]
    }
    let s = S { n, xyp };
    let mut ans = 10_000_000_000;
    for i in 0..n {
        let mut left = 0;
        let mut right = 10_000_000_000;
        while right - left > 1 {
            let mid = (left + right) / 2;
            let x = s.f(i, mid);
            if x {
                right = mid;
            } else {
                left = mid;
            }
        }
        ans = ans.min(right);
    }
    println!("{}", ans);
}
