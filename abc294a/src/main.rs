use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!(
        "{}",
        a.iter()
            .filter(|&x| x % 2 == 0)
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
