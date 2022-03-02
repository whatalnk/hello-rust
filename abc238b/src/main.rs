use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut b = [false; 360];
    b[0] = true;
    let mut p = 0;
    for x in &a {
        p += x;
        p %= 360;
        b[p] = true;
    }
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..=360 {
        if b[i % 360] {
            ans = cmp::max(ans, cur);
            cur = 0;
        }
        cur += 1;
    }
    println!("{}", ans);
}
