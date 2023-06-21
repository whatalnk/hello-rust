use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32
    }
    match a.cmp(&c) {
        Ordering::Less => {
            println!("Takahashi");
        }
        Ordering::Equal => {
            if b <= d {
                println!("Takahashi");
            } else {
                println!("Aoki");
            }
        }
        Ordering::Greater => {
            println!("Aoki");
        }
    }
}
