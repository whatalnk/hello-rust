use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        d: [i64; n],
    }
    if let Some(x) = d.iter().min() {
        println!("{}", p.min(q + x));
    }
}
