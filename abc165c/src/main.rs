use proconio::input;
use std::cell::RefCell;

struct S {
    n: usize,
    m: i64,
    aa: RefCell<Vec<Vec<i64>>>,
}

impl S {
    fn dfs(&self, a: i64, v: Vec<i64>) {
        if v.len() == self.n {
            self.aa.borrow_mut().push(v.clone());
            return;
        }
        for i in a..=self.m {
            let mut vv = v.clone();
            vv.push(i);
            self.dfs(i, vv);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: i64,
        q: usize,
        abcd: [(usize, usize, i64, i64); q]
    }
    let aa = Vec::<Vec<i64>>::new();
    let s = S {
        n: n,
        m: m,
        aa: RefCell::new(aa),
    };
    s.dfs(1, vec![]);
    let n = s.aa.borrow().len();
    let mut ans = 0;
    for i in 0..n {
        let ary = &s.aa.borrow()[i];
        let mut ans_ = 0;
        for j in 0..q {
            let (a, b, c, d) = abcd[j];
            if ary[b - 1] - ary[a - 1] == c {
                ans_ += d;
            }
        }
        ans = ans.max(ans_);
    }
    println!("{}", ans);
}
