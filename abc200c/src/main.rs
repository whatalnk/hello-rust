use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        let e = hm.entry(a[i] % 200).or_insert(vec![]);
        e.push(i);
    }
    let mut ans = 0;
    for (_, v) in &hm {
        ans += v.len() * (v.len() - 1) / 2;
    }
    println!("{}", ans);
}
