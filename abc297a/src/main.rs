use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        t: [i64; n],
    }
    for i in 0..(n - 1) {
        if t[i + 1] - t[i] <= d {
            println!("{}", t[i + 1]);
            return;
        }
    }
    println!("-1");
}
