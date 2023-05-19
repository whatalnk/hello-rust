use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    }
    if c % 2 == 0 {
        match a.abs().cmp(&b.abs()) {
            Ordering::Less => {
                println!("<");
            }
            Ordering::Equal => {
                println!("=");
            }
            Ordering::Greater => {
                println!(">");
            }
        }
    } else if a == b {
        println!("=");
    } else if (a >= 0 && a < b) || (a < 0 && (b >= 0 || a > b)) {
        println!("<");
    } else {
        println!(">");
    }
}
