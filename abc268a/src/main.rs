use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        a: [i64; 5],
    }
    let mut hs = HashSet::new();
    for ai in a.iter().take(5) {
        hs.insert(ai);
    }
    println!("{}", hs.len());
}
