use proconio::input;
use std::cmp::Ordering;
use std::println;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut takahashi = 0;
    let mut aoki = 0;
    for (x, y) in xy.iter() {
        takahashi += x;
        aoki += y;
    }
    let res = match takahashi.cmp(&aoki) {
        Ordering::Greater => "Takahashi",
        Ordering::Equal => "Draw",
        Ordering::Less => "Aoki",
    };
    println!("{}", res);
}
