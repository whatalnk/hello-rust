use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let s = n.to_string();
    let mut ans = "".to_string();
    for _ in 0..n {
        ans += &s;
    }
    println!("{}", ans);
}
