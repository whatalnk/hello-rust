use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    if n.len() == 1 {
        println!("Yes");
        return;
    }
    for i in 1..n.len() {
        if n[i - 1] <= n[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
