use std::println;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    }
    if a / 2 == b || a * 2 == b || a * 2 + 1 == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
