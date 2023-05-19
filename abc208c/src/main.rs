use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    let mut aa = Vec::new();
    for (i, item) in a.iter().enumerate() {
        aa.push((item, i));
    }
    aa.sort();
    let mut h = HashSet::new();
    let r = k / n;
    let m = k % n;
    for item in aa.iter().take(m) {
        h.insert(item.1);
    }
    for i in 0..n {
        if h.contains(&i) {
            println!("{}", r + 1);
        } else {
            println!("{}", r);
        }
    }
}
