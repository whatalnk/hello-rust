use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(i64, i64); n]
    }
    let mut bh = BinaryHeap::new();
    for i in 0..n {
        bh.push((Reverse(ab[i].0 + ab[i].1), ab[i].0, ab[i].1));
    }
    let mut ans = 0;
    for _ in 0..x {
        if let Some(e) = bh.pop() {
            match e.0 {
                Reverse(p) => {
                    ans += p;
                    if p == e.1 + e.2 {
                        bh.push((Reverse(e.2), e.1, e.1));
                    } else {
                        bh.push(e);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    println!("{}", ans);
}
