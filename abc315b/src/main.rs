use std::println;

use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [i64; m],
    }
    let dyear: i64 = d.iter().sum();
    let mid = (dyear + 1) / 2;
    let mut cur = 0;
    for (i, dd) in d.iter().enumerate() {
        cur += dd;
        if mid <= cur {
            println!("{} {}", i + 1, mid - (cur - dd));
            return;
        }
    }
    println!("{} {}", m, mid - (dyear - d[m - 1]));
}
