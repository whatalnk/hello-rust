use proconio::input;
use std::cell::Cell;

struct S {
    n: usize,
    g: Vec<Vec<bool>>,
    cd: Vec<(usize, usize)>,
    ans: Cell<bool>,
}

impl S {
    fn f(&self, used: Vec<usize>) {
        for i in 0..(self.n) {
            if !used.contains(&i) {
                let mut used_ = used.clone();
                used_.push(i);
                if used_.len() == self.n {
                    self.check(used_);
                } else {
                    self.f(used_);
                }
            }
        }
    }

    fn check(&self, used: Vec<usize>) {
        let mut ans = true;
        for (c, d) in &self.cd {
            let c_ = used[c - 1];
            let d_ = used[d - 1];
            if !self.g[c_][d_] {
                ans = false;
                break;
            }
        }
        if ans {
            self.ans.set(true);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize
    }
    let mut g = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize
        }
        g[a - 1][b - 1] = true;
        g[b - 1][a - 1] = true;
    }
    let mut cd = Vec::new();
    for _ in 0..m {
        input! {
            c: usize,
            d: usize
        }
        cd.push((c, d))
    }
    let s = S {
        n,
        g,
        cd,
        ans: Cell::new(false),
    };
    s.f(vec![]);
    if s.ans.get() {
        println!("Yes");
    } else {
        println!("No");
    }
}
