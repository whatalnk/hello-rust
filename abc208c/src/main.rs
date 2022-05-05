use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    let mut aa = Vec::new();
    for i in 0..n {
        aa.push((a[i], i));
    }
    aa.sort();
    let mut h = HashSet::new();
    let r = k / n;
    let m = k % n;
    for i in 0..m {
        h.insert(aa[i].1);
    }
    for i in 0..n {
        if h.contains(&i) {
            println!("{}", r + 1);
        } else {
            println!("{}", r);
        }
    }
}
