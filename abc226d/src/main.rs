use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut s = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            let a = x2 - x1;
            let b = y2 - y1;
            let d = num::integer::gcd(a, b);
            s.insert((a / d, b / d));
        }
    }
    println!("{}", s.len());
}
