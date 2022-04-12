use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m]
    }
    let mut hs = HashSet::new();
    for x in t {
        hs.insert(x);
    }
    for x in &s {
        if hs.contains(x) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
