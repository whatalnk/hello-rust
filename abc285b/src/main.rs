use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    for i in 1..n {
        let mut answer = false;
        'outer: for l in 0..=(n - i) {
            for k in 1..=l {
                if s[k - 1] == s[k + i - 1] {
                    println!("{}", l - 1);
                    answer = true;
                    break 'outer;
                }
            }
        }
        if !answer {
            println!("{}", n - i);
        }
    }
}
