use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    for i in 2..=16 {
        if i % 2 == 0 && s[i - 1] == '1' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
