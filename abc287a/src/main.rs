use std::println;

use proconio::input;

fn main() {
    input!(n: usize);
    let mut a = 0;
    for _ in 0..n {
        input!(s: String);
        if s == "For" {
            a += 1;
        }
    }
    if a * 2 >= n {
        println!("Yes");
    } else {
        println!("No");
    }
}
