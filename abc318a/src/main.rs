use std::println;

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        p: i64,
    }
    let mut ans = 0;
    let mut d = m;
    loop {
        if d <= n {
            ans += 1;
        } else {
            break;
        }
        d += p;
    }
    println!("{}", ans);
}
