use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        a: [i64; 5],
    }
    let mut hs = HashSet::new();
    for i in 0..5 {
        hs.insert(a[i]);
    }
    println!("{}", hs.len());
}
