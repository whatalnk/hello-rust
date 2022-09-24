use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    let mut ans = 0i64;
    let mut hm = HashMap::new();
    for r in 1..=n {
        let e = hm.entry(&b[r - 1]).or_insert(0);
        *e += 1;
        if let Some(e) = hm.get(&(b[r] - k)) {
            ans += e;
        };
    }
    println!("{}", ans);
}
