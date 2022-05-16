use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [i64; n]
    }
    let mut hs = HashMap::<i64, i64>::new();
    for i in 0..k {
        let e = hs.entry(c[i]).or_insert(0);
        *e += 1;
    }
    let mut ans = hs.len();
    for i in k..n {
        hs.entry(c[i - k]).and_modify(|e| *e -= 1);
        if hs[&c[i - k]] == 0 {
            hs.remove(&c[i - k]);
        }
        let e = hs.entry(c[i]).or_insert(0);
        *e += 1;
        ans = ans.max(hs.len());
    }
    println!("{}", ans);
}
