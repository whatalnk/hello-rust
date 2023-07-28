use std::println;

use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    if n % 5 == 0 {
        println!("{}", n);
    } else {
        let r = n / 5;
        if n - r * 5 <= (r + 1) * 5 - n {
            println!("{}", r * 5);
        } else {
            println!("{}", (r + 1) * 5);
        }
    }
}
