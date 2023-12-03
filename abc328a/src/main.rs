use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        s: [i64; n],
    }
    let ans: i64 = s.iter().filter(|ss| ss <= &&x).sum();
    println!("{:?}", ans);
}
