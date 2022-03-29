use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
       n: usize
    }
    let mut hs = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            a: [i32; l]
        }
        hs.insert(a);
    }
    println!("{}", hs.len());
}
