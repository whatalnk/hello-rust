use std::println;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let mut aa = 1;
    let mut bb = 1;
    for _ in 0..(a as usize) {
        bb *= b;
    }
    for _ in 0..(b as usize) {
        aa *= a;
    }
    println!("{}", aa + bb);
}
