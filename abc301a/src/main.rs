use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let m = match n % 2 {
        0 => n / 2,
        _ => n / 2 + 1,
    };

    let mut c1 = 0;
    let mut c2 = 0;
    for si in &s {
        if si == &'T' {
            c1 += 1;
        } else {
            c2 += 1;
        }
        if c1 >= m {
            println!("T");
            return;
        } else if c2 >= m {
            println!("A");
            return;
        }
    }
}
