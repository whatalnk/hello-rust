use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut h = HashMap::new();
    for k in &s {
        let v = h.entry(k).or_insert(0);
        *v += 1;
    }
    let mut cnt = 0;
    let mut ans = "";
    for (k, v) in &h {
        if v > &cnt {
            cnt = *v;
            ans = k;
        }
    }
    println!("{}", ans);
}
