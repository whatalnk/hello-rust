use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: i64,
        x: i64,
        p: [i64; n],
    }
    for (i, pi) in p.iter().enumerate() {
        if h + pi >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
