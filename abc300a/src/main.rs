use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        c: [i64; n]
    }
    for (i, ci) in c.iter().enumerate() {
        if ci == &(a + b) {
            println!("{}", i + 1);
            return;
        }
    }
}
