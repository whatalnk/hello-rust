use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [usize; n]
    }
    let mut hm = HashMap::<&i64, i64>::new();
    for x in &a {
        let e = hm.entry(x).or_insert(0);
        *e += 1;
    }
    let mut ans = 0;
    for j in c {
        match hm.get(&b[j - 1]) {
            Some(v) => ans += v,
            None => ans += 0,
        }
    }
    println!("{}", ans);
}
