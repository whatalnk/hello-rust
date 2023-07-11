use std::{println, vec};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!(s: Chars);
    let mut ret = vec![];
    for c in &s {
        if c == &'1' {
            ret.push('0');
        } else {
            ret.push('1');
        }
    }
    println!("{}", ret.iter().collect::<String>());
}
