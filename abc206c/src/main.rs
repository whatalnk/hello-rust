use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut hm = HashMap::<&i64, i64>::new();
    for x in &a {
        let e = hm.entry(x).or_insert(0);
        *e += 1;
    }
    let mut ans = (n * (n - 1) / 2) as i64;
    for (_, v) in &hm {
        ans -= v * (v - 1) / 2;
    }
    println!("{}", ans);
}
