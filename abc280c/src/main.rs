use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    s.push(' ');
    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
}
