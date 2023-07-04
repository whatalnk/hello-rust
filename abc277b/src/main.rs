use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let a = ['H', 'C', 'D', 'S'];
    let b = [
        'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
    ];
    let mut hs = HashSet::new();
    for si in s.iter().take(n) {
        let c = si;
        if a.contains(&c[0]) && b.contains(&c[1]) {
            hs.insert(c);
        }
    }
    if hs.len() == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
