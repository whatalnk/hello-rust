use std::{collections::HashSet, println};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }
    let mut hst = HashSet::new();
    for ti in t.iter() {
        hst.insert(ti.as_str());
    }
    let mut ans = 0;
    for si in s.iter() {
        if hst.contains(&si[3..6]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
