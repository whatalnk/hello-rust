use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: i32
    }
    let mut m = vec![HashSet::<char>::new(); n];
    for i in 0..n {
        input! {
            s: Chars
        }
        for c in s {
            m[i].insert(c);
        }
    }
    let a = "abcdefghijklmnopqrstuvwxyz".chars();
    let mut ans = 0;
    for c in a {
        let mut cnt = 0;
        for i in 0..n {
            if m[i].contains(&c) {
                cnt += 1;
            }
        }
        if cnt >= k {
            ans += 1;
        }
    }
    println!("{}", ans);
}
