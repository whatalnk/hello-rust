use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    if (s[0] == s[1]) && (s[1] == s[2]) {
        println!("1");
    } else if (s[0] == s[1]) || (s[1] == s[2]) || (s[2] == s[0]) {
        println!("3");
    } else {
        println!("6");
    }
}
