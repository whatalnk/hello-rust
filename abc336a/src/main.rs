use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut s = "L".to_string();
    for _ in 0..n {
        s.push('o');
    }
    println!("{}ng", s);
}
