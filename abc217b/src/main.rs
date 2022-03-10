use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: [String; 3]
    }
    let mut set = HashSet::<String>::new();
    for x in &["ABC", "ARC", "AGC", "AHC"] {
        set.insert(x.to_string());
    }
    for x in &s {
        set.remove(x);
    }
    for x in &set {
        println!("{}", x);
    }
}
