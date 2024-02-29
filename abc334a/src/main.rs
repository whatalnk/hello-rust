use std::println;

use proconio::input;

fn main() {
    input! {
        b: i64,
        g: i64,
    }
    if b > g {
        println!("Bat");
    } else {
        println!("Glove");
    }
}
