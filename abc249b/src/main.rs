use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars
    }
    let mut hm = HashMap::new();
    for k in s {
        let e = hm.entry(k).or_insert(0);
        *e += 1;
    }
    let mut lower = false;
    let mut upper = false;
    let mut unique = true;
    for c in hm.keys() {
        if c.is_ascii_lowercase() {
            lower = true;
        }
        if c.is_ascii_uppercase() {
            upper = true
        }
        if hm[c] > 1 {
            unique = false;
        }
    }
    if lower && upper && unique {
        println!("Yes");
    } else {
        println!("No");
    }
}
