use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: i32,
        x: i32,
        a: [i32; n]
    }
    let mut h = BinaryHeap::new();
    let mut ans = 0;
    for i in a {
        ans += i;
        h.push(i);
    }
    for _ in 0..k {
        if let Some(v) = h.pop() {
            if v > x {
                h.push(v - x);
                ans -= x;
            } else {
                ans -= v;
            }
        } else {
            break;
        }
    }
    println!("{}", ans);
}
