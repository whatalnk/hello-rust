use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [usize; m],
    }
    println!("{}", b.iter().fold(0, |acc, i| acc + a[*i - 1]));
}
