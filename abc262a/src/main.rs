use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        y: i64,
    }
    let r = y % 4;
    match r.cmp(&2) {
        Ordering::Less => {
            println!("{}", y + 2 - r);
        }
        Ordering::Equal => {
            println!("{}", y);
        }
        Ordering::Greater => {
            println!("{}", y + 3);
        }
    }
}
