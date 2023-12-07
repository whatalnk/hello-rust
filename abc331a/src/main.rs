use std::println;

use proconio::input;

fn main() {
    input! {
        m: i64,
        d: i64,
        yyyy: i64,
        mm: i64,
        dd: i64,
    }
    let mut yyyy_next = yyyy;
    let mut mm_next = mm;
    let mut dd_next = dd;
    dd_next += 1;
    if dd_next > d {
        dd_next -= d;
        mm_next += 1;
    }
    if mm_next > m {
        mm_next -= m;
        yyyy_next += 1;
    }
    println!("{} {} {}", yyyy_next, mm_next, dd_next);
}
