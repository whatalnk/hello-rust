use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut ans = VecDeque::new();
    ans.push_back(n.to_string());
    for i in 0..n {
        if s[n - i - 1] == 'R' {
            ans.push_front((n - i - 1).to_string());
        } else {
            ans.push_back((n - i - 1).to_string());
        }
    }
    println!("{}", ans.iter().cloned().collect::<Vec<_>>().join(" "));
}
