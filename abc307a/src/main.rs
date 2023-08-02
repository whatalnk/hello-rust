use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n * 7],
    }
    let mut ans = vec![];
    for i in 0..n {
        let mut c = 0;
        for j in 0..7 {
            c += a[i * 7 + j];
        }
        ans.push(c.to_string());
    }
    println!("{}", ans.join(" "));
}
