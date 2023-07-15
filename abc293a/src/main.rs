use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }
    for i in 1..=(s.len() / 2) {
        s.swap(2 * i - 2, 2 * i - 1);
    }
    println!("{}", s.iter().map(|c| c.to_string()).collect::<String>());
}
