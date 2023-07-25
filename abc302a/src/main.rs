use std::println;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let c = a / b;
    let cc = a % b;
    if cc != 0 {
        println!("{}", c + 1);
    } else {
        println!("{}", c);
    }
}
