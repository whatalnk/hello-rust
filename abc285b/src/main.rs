use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    for i in 1..n {
        let mut ans = 0;
        for k in 1..=(n - i) {
            if s[k - 1] != s[k + i - 1] {
                ans += 1;
            } else {
                break;
            }
        }
        println!("{}", ans);
    }
}
