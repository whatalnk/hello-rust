use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        k: i64,
        pq: [(i64, i64); n],
    }
    let mut amount = 0;
    for (p, q) in &pq {
        amount += p * q;
    }
    if amount < s {
        amount += k;
    }
    println!("{}", amount);
}
