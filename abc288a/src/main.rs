use std::println;

use proconio::input;

fn main() {
    input!(n: usize);
    for _ in 0..n {
        input!(a: i64, b: i64);
        println!("{}", a + b);
    }
}
