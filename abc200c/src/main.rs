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
        for i in 0..(v.len()) {
            for j in (i + 1)..(v.len()) {
                if (a[v[i]] - a[v[j]]) % 200 == 0 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
