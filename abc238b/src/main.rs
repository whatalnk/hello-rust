use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }
    let mut b = Vec::<i32>::new();
    for x in &a {
        for i in 0..b.len() {
            b[i] += x;
            b[i] %= 360;
        }
        b.push(*x);
    }
    b.push(360);
    b.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    for i in 0..(b.len() - 1) {
        ans = cmp::max(ans, b[i] - b[i + 1]);
    }
    println!("{}", ans);
}
