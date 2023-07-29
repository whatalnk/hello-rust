use std::{println, vec};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars
    }
    let mut ss = vec![];
    for c in &s {
        ss.push(c.to_string());
        ss.push(c.to_string());
    }
    println!("{}", ss.join(""));
}
