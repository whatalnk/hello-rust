use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut ss1 = "".to_string();
    let mut ss2 = "".to_string();
    for _ in 0..(n / 2) {
        ss1.push_str("MF");
        ss2.push_str("FM");
    }
    if n % 2 != 0 {
        ss1.push_str("M");
        ss2.push_str("F");
    }
    if s == ss1 || s == ss2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
