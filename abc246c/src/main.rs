use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        x: i64,
        a: [i64; n]
    }
    let mut h = BinaryHeap::new();
    let mut ans = 0;
    for i in a {
        ans += i;
        h.push(i);
    }
    while k > 0 {
        if let Some(v) = h.pop() {
            if v > x {
                let mut m = v / x;
                m = m.min(k);
                h.push(v % x);
                ans -= x * m;
                k -= m;
            } else {
                ans -= v;
                k -= 1;
            }
        } else {
            break;
        }
    }
    println!("{}", ans);
}
