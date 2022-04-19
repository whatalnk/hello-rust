use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize
    }
    let mut tt = vec![];
    let mut aa = vec![];
    for _ in 0..n {
        input! {
            t: i128,
            k: usize,
            a: [usize; k]
        }
        tt.push(t);
        aa.push(a);
    }
    let mut hs = HashSet::new();
    let mut q = vec![];
    hs.insert(n);
    q.push(n);
    while q.len() > 0 {
        let i = q.pop().unwrap();
        for a in &aa[i - 1] {
            if !hs.contains(a) {
                hs.insert(*a);
                if a != &1 {
                    q.push(*a);
                }
            }
        }
    }
    let mut ans: i128 = 0;
    for i in &hs {
        ans += tt[*i - 1];
    }
    println!("{}", ans);
}
