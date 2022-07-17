use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    if s[0] == s[1] && s[1] == s[2] {
        println!("-1");
    } else if s[0] == s[1] {
        println!("{}", s[2]);
    } else if s[1] == s[2] {
        println!("{}", s[0]);
    } else if s[0] == s[2] {
        println!("{}", s[1]);
    } else {
        println!("{}", s[0]);
    }
}
