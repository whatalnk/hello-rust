use std::println;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    }
    match (a, b) {
        (1, 2) | (2, 3) | (4, 5) | (5, 6) | (7, 8) | (8, 9) => println!("Yes"),
        _ => println!("No"),
    };
}
