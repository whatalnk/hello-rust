use std::cmp::Ordering;
use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    }
    if let Some(pmax) = p.iter().take(n).skip(1).max() {
        match p[0].cmp(pmax) {
            Ordering::Greater => println!("0"),
            Ordering::Equal => println!("1"),
            Ordering::Less => println!("{}", pmax - p[0] + 1),
        }
    } else {
        println!("0");
    }
}
