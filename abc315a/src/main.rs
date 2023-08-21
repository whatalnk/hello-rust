use std::{println, vec};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let aeiou = ['a', 'e', 'i', 'o', 'u'];
    let mut ans = vec![];
    for c in &s {
        if !aeiou.contains(c) {
            ans.push(c);
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<String>());
}
