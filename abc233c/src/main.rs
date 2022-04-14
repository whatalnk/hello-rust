use proconio::input;
use std::cell::Cell;

struct S {
    n: usize,
    aa: Vec<Vec<i64>>,
    ans: Cell<i64>,
}

impl S {
    fn f(&self, i: usize, p: i64) {
        for a in &self.aa[i] {
            if i == self.n - 1 && p == *a {
                self.ans.set(self.ans.get() + 1);
            } else if i < self.n - 1 && p % a == 0 {
                self.f(i + 1, p / a);
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        x: i64
    }
    let mut ll = vec![];
    let mut aa = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            a: [i64; l]
        }
        ll.push(l);
        aa.push(a);
    }
    let s = S {
        n: n,
        aa: aa,
        ans: Cell::new(0),
    };
    s.f(0, x);
    println!("{}", s.ans.get());
}
