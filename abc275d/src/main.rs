use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

struct S {
    v: BTreeSet<i64>,
}
impl S {
    fn new() -> S {
        S {
            v: BTreeSet::<i64>::new(),
        }
    }
    fn f(&mut self, n: i64) {
        if !self.v.contains(&(n / 2)) {
            self.v.insert(n / 2);
            self.f(n / 2);
        }
        if !self.v.contains(&(n / 3)) {
            self.v.insert(n / 3);
            self.f(n / 3);
        }
    }
}

fn main() {
    input! {
        n: i64,
    }
    let mut s = S::new();
    s.f(n);
    let mut v: Vec<&i64> = s.v.iter().collect();
    v.push(&n);
    let mut memo = BTreeMap::<i64, i64>::new();
    memo.insert(0, 1);
    for i in 0..v.len() {
        if v[i] > &0 {
            let k1 = v[i] / 2;
            let v1 = memo.get(&k1).copied().unwrap();
            let k2 = v[i] / 3;
            let v2 = memo.get(&k2).copied().unwrap();
            memo.insert(v[i].clone(), v1 + v2);
        }
    }
    println!("{}", memo[&n]);
}
