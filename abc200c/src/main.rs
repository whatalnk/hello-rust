use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut hm = HashMap::new();
    for (i, item) in a.iter().enumerate() {
        let e = hm.entry(item % 200).or_insert_with(|| vec![]);
        e.push(i);
    }
    let mut ans = 0;
    for v in hm.values() {
        ans += v.len() * (v.len() - 1) / 2;
    }
    println!("{}", ans);
}
