use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars
    }
    let mut iter = s.split(|c| c == &'|');
    if let Some(x) = iter.nth(1) {
        if x.contains(&'*') {
            println!("in");
            return;
        }
    }
    println!("out");
}
