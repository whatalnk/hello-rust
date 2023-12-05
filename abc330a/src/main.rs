use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n],
    }
    println!(
        "{}",
        a.into_iter()
            .filter(|x| x >= &l)
            .collect::<Vec<i64>>()
            .len()
    );
}
