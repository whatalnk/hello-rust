use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    for i in 0..n {
        if s[i] == t[i]
            || (s[i] == '1' && t[i] == 'l')
            || (s[i] == 'l' && t[i] == '1')
            || (s[i] == '0' && t[i] == 'o')
            || (s[i] == 'o' && t[i] == '0')
        {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
