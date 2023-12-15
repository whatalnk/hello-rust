use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        k: usize,
        s: Chars,
    }
    let mut ans = vec![];
    let mut c = 0;
    for si in s.iter() {
        if si == &'o' && c < k {
            ans.push('o'.to_string());
            c += 1;
        } else {
            ans.push('x'.to_string());
        }
    }
    println!("{}", ans.join(""));
}
