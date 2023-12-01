use std::println;

use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64
    }
    if y > x {
        if y - x > 2 {
            println!("No");
        } else {
            println!("Yes");
        }
    } else if x - y > 3 {
        println!("No");
    } else {
        println!("Yes");
    }
}
