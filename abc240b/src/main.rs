use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut h = HashSet::new();
    for k in &a {
        h.insert(k);
    }
    println!("{}", h.len());
}
