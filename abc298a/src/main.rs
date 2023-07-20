use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars
    }
    if s.contains(&'o') && !s.contains(&'x') {
        println!("Yes");
    } else {
        println!("No");
    }
}
