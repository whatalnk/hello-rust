use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut ans = Vec::<char>::new();
    for i in 1..n {
        if s[i - 1] == 'n' && s[i] == 'a' {
            ans.push('n');
            ans.push('y');
        } else {
            ans.push(s[i - 1]);
        }
    }
    ans.push(s[n - 1]);
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
