use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        st: [(String, i64); n]
    }
    let mut hs = HashSet::new();
    let mut ans = 0;
    for i in 0..n {
        if !hs.contains(&st[i].0) {
            hs.insert(&st[i].0);
            if st[i].1 > st[ans].1 {
                ans = i;
            }
        }
    }
    println!("{}", ans + 1);
}
